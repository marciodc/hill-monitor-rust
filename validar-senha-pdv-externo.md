# Skill — Validar senha do operador no PDV externo (scrypt)

> **Briefing pra outra IA / dev:** como validar a senha digitada pelo
> operador no PDV externo (Cardswap, SiTef, Java, .NET, Delphi, etc)
> usando o mesmo algoritmo que a retaguarda Onix usou pra gerar o hash.
>
> A retaguarda envia `senha_pdv_hash` no bloco `cadastros.usuarios_pdv[]`
> do sync (snapshot e delta). O PDV local guarda esse hash. Quando o
> operador digita a senha pra abrir a venda / autorizar uma ação, o PDV
> valida **offline** usando `scrypt` — sem chamar o servidor.
>
> **Zero senha em claro trafega ou fica armazenada.**

---

## Sumário

1. [Por que scrypt](#1-por-que-scrypt)
2. [Formato do hash](#2-formato-do-hash)
3. [Algoritmo de validação](#3-algoritmo-de-validação)
4. [Parâmetros exatos](#4-parâmetros-exatos)
5. [Exemplos por linguagem](#5-exemplos-por-linguagem)
6. [Testes obrigatórios](#6-testes-obrigatórios)
7. [Armadilhas comuns](#7-armadilhas-comuns)
8. [FAQ](#8-faq)

---

## 1. Por que scrypt

`scrypt` (RFC 7914) é uma função de derivação de chave (KDF) projetada
pra ser **cara em memória**, dificultando ataques por hardware
dedicado (GPU, ASIC). Padrão do OWASP pra hash de senhas quando bcrypt
ou Argon2 não estão disponíveis.

Alternativas:

- `bcrypt` — bom mas parâmetros fixos, mais suscetível a GPU
- `Argon2id` — mais moderno, mas suporte irregular em Delphi/.NET legacy
- `PBKDF2` — barato demais em GPU
- `scrypt` ✅ — escolhido: suporte amplo (Node crypto nativo, .NET via
  lib, Delphi via BouncyCastle, Java nativo desde 15)

---

## 2. Formato do hash

O `senha_pdv_hash` vem no sync com **6 campos separados por `$`**:

```
scrypt$N$r$p$saltHex$hashHex
```

Exemplo real:

```
scrypt$16384$8$1$3f7a91c4e0b8d2f5a6e9c1b7d3f8a4e2$b8f2c1e4d9a6f0b3c7e1d5a8f4b2c9e6d0a3f7b1c5e8d2a6f9b4c0e3d7a1f5b8c2e6d9a4f0b3c7e1d5a8f4b2c9e6d0a3f7b1c5e8d2a6f9b4c0e3d7a1f5b8c2e6d9a4
```

**Decomposição:**

| Índice | Campo       | Tipo                       | Descrição                     |
| ------ | ----------- | -------------------------- | ----------------------------- |
| 0      | `scrypt`    | literal                    | identificador do algoritmo    |
| 1      | `16384`     | inteiro                    | parâmetro N (cost factor)     |
| 2      | `8`         | inteiro                    | parâmetro r (block size)      |
| 3      | `1`         | inteiro                    | parâmetro p (parallelization) |
| 4      | `3f7a91...` | hex (32 chars = 16 bytes)  | salt aleatório                |
| 5      | `b8f2c1...` | hex (128 chars = 64 bytes) | hash derivado                 |

**Regra:** se o hash não tem exatamente 6 campos ou o [0] não é
`"scrypt"`, considere formato inválido → rejeita.

---

## 3. Algoritmo de validação

Fluxo em pseudocódigo:

```
function validarSenha(senhaDigitada: string, hashArmazenado: string): boolean {
  // 1. Parse do hash
  partes = split(hashArmazenado, "$")
  if (len(partes) != 6) return false
  if (partes[0] != "scrypt") return false

  N       = parseInt(partes[1])       // 16384
  r       = parseInt(partes[2])       // 8
  p       = parseInt(partes[3])       // 1
  salt    = hexToBytes(partes[4])     // 16 bytes
  esperado = hexToBytes(partes[5])    // 64 bytes

  // 2. Deriva a chave da senha digitada usando MESMO salt e MESMOS parâmetros
  derivado = scrypt(
    senha    = senhaDigitada,          // UTF-8 bytes (importante!)
    salt     = salt,
    N        = N,
    r        = r,
    p        = p,
    tamanho  = len(esperado)           // 64 bytes
  )

  // 3. Compara com timing-safe (previne timing attack)
  return timingSafeEqual(derivado, esperado)
}
```

**Ponto crítico:** o comparador precisa ser **constant-time** (sempre
percorre todos os bytes independente de match). Comparação `==` normal
vaza informação por timing (retorna cedo no primeiro byte diferente).

---

## 4. Parâmetros exatos

Estes são os valores que a retaguarda Onix usa hoje:

| Parâmetro    | Valor                                          | Significado                                       |
| ------------ | ---------------------------------------------- | ------------------------------------------------- |
| **N**        | `16384` (2^14)                                 | Cost factor. Cada +1 no expoente = 2× mais lento. |
| **r**        | `8`                                            | Block size.                                       |
| **p**        | `1`                                            | Parallelization.                                  |
| **Salt**     | 16 bytes aleatórios (`crypto.randomBytes(16)`) | Único por senha.                                  |
| **KeyLen**   | `64` bytes                                     | Tamanho do hash derivado.                         |
| **Encoding** | UTF-8 na senha, hex no salt e hash             | Consistente com Node.js/browser.                  |

**IMPORTANTE:** você **NÃO precisa fixar** esses valores no seu código
do PDV — pegue tudo do próprio hash armazenado (`partes[1]/[2]/[3]`).
Isso permite a retaguarda subir `N` no futuro sem quebrar o PDV.

Latência esperada com esses parâmetros:

- **Servidor Node** (v22, Intel/AMD): ~30-50ms
- **PDV Windows** (i3 típico): ~50-100ms
- **PDV Android** (ARM médio): ~100-200ms

Aceitável pra digitar senha uma vez por venda.

---

## 5. Exemplos por linguagem

### 5.1 Delphi (bouncy castle / delphipraxis)

Delphi não tem scrypt na RTL. Usar **DelphiCryptoLib** (fork brasileiro
do BouncyCastle) — MIT license, funciona Windows/Linux/macOS.

```delphi
uses
  ClpConverters,
  ClpIScrypt,
  ClpScrypt,
  System.SysUtils, System.StrUtils;

function ValidarSenhaPdv(const SenhaDigitada, HashArmazenado: string): Boolean;
var
  Partes: TArray<string>;
  N, R, P, KeyLen: Integer;
  Salt, Esperado, Derivado: TBytes;
  I: Integer;
  Diff: Byte;
begin
  Result := False;

  // 1. Parse formato scrypt$N$r$p$saltHex$hashHex
  Partes := SplitString(HashArmazenado, '$');
  if Length(Partes) <> 6 then Exit;
  if Partes[0] <> 'scrypt' then Exit;

  try
    N := StrToInt(Partes[1]);
    R := StrToInt(Partes[2]);
    P := StrToInt(Partes[3]);
    Salt := THex.Decode(Partes[4]);
    Esperado := THex.Decode(Partes[5]);
    KeyLen := Length(Esperado);
  except
    Exit;
  end;

  // 2. Deriva usando MESMOS parâmetros do hash
  Derivado := TScrypt.Generate(
    TEncoding.UTF8.GetBytes(SenhaDigitada),
    Salt,
    N, R, P, KeyLen);

  // 3. Comparação timing-safe (XOR + OR acumulado)
  if Length(Derivado) <> Length(Esperado) then Exit;
  Diff := 0;
  for I := 0 to Length(Derivado) - 1 do
    Diff := Diff or (Derivado[I] xor Esperado[I]);

  Result := Diff = 0;
end;
```

**Instalação da lib:** `git clone https://github.com/Xor-el/CryptoLib4Pascal`
e adicionar ao Search Path. Suporta Delphi XE7+ e FPC 3.2+.

---

### 5.2 C# / .NET (Konscious.Security.Cryptography)

.NET não tem scrypt nativo. Usar **Konscious.Security.Cryptography.Scrypt**
(NuGet). Alternativa: `libsodium-net`.

```csharp
using System;
using System.Text;
using Konscious.Security.Cryptography;

public static bool ValidarSenhaPdv(string senhaDigitada, string hashArmazenado)
{
    // 1. Parse formato scrypt$N$r$p$saltHex$hashHex
    var partes = hashArmazenado.Split('$');
    if (partes.Length != 6) return false;
    if (partes[0] != "scrypt") return false;

    int N, r, p;
    byte[] salt, esperado;
    try
    {
        N    = int.Parse(partes[1]);
        r    = int.Parse(partes[2]);
        p    = int.Parse(partes[3]);
        salt = Convert.FromHexString(partes[4]);
        esperado = Convert.FromHexString(partes[5]);
    }
    catch { return false; }

    // 2. Deriva com MESMOS parâmetros
    using var scrypt = new Scrypt(
        password: Encoding.UTF8.GetBytes(senhaDigitada),
        salt:     salt,
        cost:     N,
        blockSize: r,
        parallel:  p);

    var derivado = scrypt.GetBytes(esperado.Length);

    // 3. Timing-safe compare
    return CryptographicOperations.FixedTimeEquals(derivado, esperado);
}
```

`FixedTimeEquals` disponível em .NET 6+. Pra .NET Framework 4.x, usa
loop manual XOR igual o Delphi.

**Instalação:** `dotnet add package Konscious.Security.Cryptography.Scrypt`

---

### 5.3 Java / Kotlin

Java tem `SCryptParameters` nativo desde JDK 15. Antes disso, usa
BouncyCastle (`bcprov`).

```java
import org.bouncycastle.crypto.generators.SCrypt;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;

public class SenhaPdvValidator {

    public static boolean validar(String senhaDigitada, String hashArmazenado) {
        // 1. Parse
        String[] partes = hashArmazenado.split("\\$");
        if (partes.length != 6) return false;
        if (!partes[0].equals("scrypt")) return false;

        int N, r, p;
        byte[] salt, esperado;
        try {
            N = Integer.parseInt(partes[1]);
            r = Integer.parseInt(partes[2]);
            p = Integer.parseInt(partes[3]);
            salt = hexToBytes(partes[4]);
            esperado = hexToBytes(partes[5]);
        } catch (Exception e) { return false; }

        // 2. Deriva
        byte[] derivado = SCrypt.generate(
            senhaDigitada.getBytes(StandardCharsets.UTF_8),
            salt, N, r, p, esperado.length);

        // 3. Timing-safe compare — MessageDigest.isEqual é constant-time
        return MessageDigest.isEqual(derivado, esperado);
    }

    private static byte[] hexToBytes(String hex) {
        int len = hex.length();
        byte[] out = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            out[i / 2] = (byte) ((Character.digit(hex.charAt(i), 16) << 4)
                              + Character.digit(hex.charAt(i + 1), 16));
        }
        return out;
    }
}
```

**Maven dep:**

```xml
<dependency>
  <groupId>org.bouncycastle</groupId>
  <artifactId>bcprov-jdk18on</artifactId>
  <version>1.78</version>
</dependency>
```

---

### 5.4 Python

```python
import hashlib
import hmac

def validar_senha_pdv(senha_digitada: str, hash_armazenado: str) -> bool:
    # 1. Parse
    partes = hash_armazenado.split("$")
    if len(partes) != 6 or partes[0] != "scrypt":
        return False

    try:
        n = int(partes[1])
        r = int(partes[2])
        p = int(partes[3])
        salt = bytes.fromhex(partes[4])
        esperado = bytes.fromhex(partes[5])
    except (ValueError, IndexError):
        return False

    # 2. Deriva usando hashlib.scrypt (nativo desde Python 3.6)
    derivado = hashlib.scrypt(
        password=senha_digitada.encode("utf-8"),
        salt=salt,
        n=n, r=r, p=p,
        dklen=len(esperado),
        maxmem=128 * n * r * 2  # margem de segurança
    )

    # 3. Timing-safe (hmac.compare_digest é constant-time)
    return hmac.compare_digest(derivado, esperado)
```

Nada de dependência externa — `hashlib` e `hmac` são stdlib.

---

### 5.5 JavaScript / Node.js

Se o cliente PDV é Electron/Node ou browser (subtle crypto).

**Node.js** (nativo):

```javascript
const { scryptSync, timingSafeEqual } = require("node:crypto");

function validarSenhaPdv(senhaDigitada, hashArmazenado) {
  // 1. Parse
  const partes = hashArmazenado.split("$");
  if (partes.length !== 6 || partes[0] !== "scrypt") return false;

  let N, r, p, salt, esperado;
  try {
    N = parseInt(partes[1], 10);
    r = parseInt(partes[2], 10);
    p = parseInt(partes[3], 10);
    salt = Buffer.from(partes[4], "hex");
    esperado = Buffer.from(partes[5], "hex");
  } catch {
    return false;
  }

  // 2. Deriva
  const derivado = scryptSync(
    senhaDigitada, // Node aceita string UTF-8 direto
    salt,
    esperado.length,
    { N, r, p }
  );

  // 3. Timing-safe
  if (derivado.length !== esperado.length) return false;
  return timingSafeEqual(derivado, esperado);
}
```

Zero dependência externa.

**Browser** (via `scrypt-js`):

```bash
npm install scrypt-js
```

```javascript
import { scrypt } from "scrypt-js";

async function validarSenhaPdv(senhaDigitada, hashArmazenado) {
  const partes = hashArmazenado.split("$");
  if (partes.length !== 6 || partes[0] !== "scrypt") return false;

  const N = parseInt(partes[1], 10);
  const r = parseInt(partes[2], 10);
  const p = parseInt(partes[3], 10);
  const salt = Uint8Array.from(
    partes[4].match(/.{2}/g).map(b => parseInt(b, 16))
  );
  const esperado = Uint8Array.from(
    partes[5].match(/.{2}/g).map(b => parseInt(b, 16))
  );

  const senhaBytes = new TextEncoder().encode(senhaDigitada);
  const derivado = await scrypt(senhaBytes, salt, N, r, p, esperado.length);

  // Timing-safe manual (Web Crypto não expõe helper direto)
  if (derivado.length !== esperado.length) return false;
  let diff = 0;
  for (let i = 0; i < derivado.length; i++) diff |= derivado[i] ^ esperado[i];
  return diff === 0;
}
```

---

## 6. Testes obrigatórios

Antes de considerar a implementação pronta, valide com estes **6 casos**:

### 6.1 Vetor conhecido

Gere um hash na retaguarda pra a senha `"123456"` (usando `hashPassword`
do Onix) e valide pelo PDV. Exemplo do que virá:

```
Senha:  "123456"
Hash:   "scrypt$16384$8$1$a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6$8f7e6d5c4b3a291a8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0c9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0c9d8"
Espera: TRUE
```

Cada hash é único (salt aleatório), então rode a validação e confira
que retorna `TRUE`.

### 6.2 Senha errada

Com o mesmo hash acima, validar `"654321"` deve retornar **FALSE**.

### 6.3 Hash malformado (deve rejeitar sem crashar)

```
"scrypt$16384$8$1$onlyOneSaltMissingHash"    → false
"invalid$16384$8$1$ab$cd"                     → false
""                                             → false
"scrypt"                                       → false
null / undefined                               → false
```

### 6.4 Case-sensitive

`"MinhaSenha"` e `"minhasenha"` devem gerar hashes diferentes e
validações opostas. Confirma que o encoding não uppercase/lowercase.

### 6.5 Caracteres especiais

Senha com acentos, emojis e espaços:

```
"Senha com espaço"     → deve validar
"café☕2026"           → deve validar
"pass@word!#"          → deve validar
```

Confirma que UTF-8 tá certo no encoding.

### 6.6 Comparação constant-time

Se possível, faça teste comportamental: valide 1000× uma senha certa e
1000× uma errada, meça tempo médio. Deve ser **estatisticamente igual**
(±5% de variação). Se a errada retorna consistentemente mais rápido,
sua comparação não é constant-time — corrija.

---

## 7. Armadilhas comuns

### ❌ Comparação com `==` ou `.equals()`

**Errado** — vaza timing:

```javascript
return derivado.toString("hex") === esperado.toString("hex");
```

Isso retorna cedo no primeiro byte diferente, atacante mede latência e
descobre bytes um a um. Sempre use `timingSafeEqual`/`FixedTimeEquals`/
`MessageDigest.isEqual`/`hmac.compare_digest`.

### ❌ Encoding errado da senha

Senha `"café"` vira bytes diferentes em UTF-8 (`63 61 66 c3 a9`) vs
Latin-1 (`63 61 66 e9`). Sempre **UTF-8**. Java precisa
`.getBytes(StandardCharsets.UTF_8)`; .NET precisa `Encoding.UTF8`.

### ❌ Hardcodear N/r/p do PDV

Não faça `if (N != 16384) return false`. Pegue os valores do próprio
hash. Se a retaguarda subir `N` no futuro (segurança evolui), seu PDV
continua funcionando sem mudança.

### ❌ Guardar a senha em variável muito tempo

Depois de validar, **zere a string da senha em memória** (se a linguagem
permitir). Delphi: `FillChar(SenhaDigitada[1], Length(SenhaDigitada), 0);`.
.NET: `SecureString`. Java: `char[]` em vez de `String`.

### ❌ Aceitar hash sem prefixo `scrypt$`

Se aparecer hash em outro formato (bcrypt `$2y$`, argon2 `$argon2id$`,
sha256 puro), rejeitar. Retaguarda garante que só envia `scrypt$` hoje.

### ❌ Cachear resultado de validação em disco

Não escreva "senha validada em 2026-08-04" em arquivo. Cada abertura
de venda **revalida do zero** — protege contra máquina roubada onde
alguém pega o hash local + arquivo de cache e cria login perpétuo.

### ❌ Enviar o hash em log

Log `usuario=jose senha_hash=scrypt$16384...` = risco. Se o log vazar,
atacante pode brute-forcear offline. **Nunca logue o `senha_pdv_hash`**.

### ❌ Retornar `TRUE` quando hash é NULL

Usuário sem senha cadastrada (`senha_pdv_hash: null` no sync) deve
**BLOQUEAR o acesso** (não permitir venda), não liberar. Retorne FALSE
sempre que hash é ausente.

### ❌ Consultar servidor pra validar

Você tem o hash local. Valide localmente. Consultar servidor toda vez
que o operador digita senha viola o "PDV opera offline" e cria pico de
requests. Servidor só é consultado no sync (uma vez a cada 30s) —
validação de senha nunca precisa ir online.

---

## 8. FAQ

### Se o operador esquece a senha, como resetar?

Gerente/admin altera no backoffice Onix (Users → Editar → Nova senha
PDV). Backoffice gera hash novo e no próximo sync o PDV recebe o
`senha_pdv_hash` atualizado. Instantâneo (max 30s de latência de sync).

### Posso trocar scrypt por Argon2 no futuro?

Sim. O formato prefixado (`scrypt$...` vs `argon2id$...`) foi escolhido
justamente pra permitir migração. Retaguarda pode gerar hashes novos
em Argon2 e o PDV precisa detectar o prefixo e chamar a lib correta.
Hashes antigos scrypt continuam válidos até rehash na próxima
alteração de senha.

### O que acontece se o cliente PDV está offline há dias e um user novo foi criado?

Enquanto não sincroniza, o PDV não conhece o user novo — não vai
autenticar. Sync a cada 30-60s resolve na primeira reconexão. Isso é
esperado — PDV offline vê estado congelado.

### E se dois operadores tiverem a mesma senha?

Cada `hashPassword` gera **salt aleatório novo**. Mesma senha = hashes
DIFERENTES. Isso é bom — protege contra ataque de rainbow table
comparando hashes iguais.

### Como faço lockout após N tentativas erradas?

Não é responsabilidade do algoritmo — é do fluxo de UI. Sugestão:
contador local no PDV, se `tentativas_erradas >= 5` bloqueia por 5min
(pode inclusive gravar em log local pra auditar). O backend não sabe
das tentativas porque validação é offline.

### Qual o custo se atacante roubar o hash?

Com `N=16384, r=8, p=1`, cada tentativa de brute-force custa ~50ms
CPU. Pra senha 6 dígitos (1M combinações) = ~14h em 1 core, ~1h em
16 cores. Pra senha alfanumérica 8 chars (~200T combinações) = **300+
anos**. Recomendação: exija senha 6+ chars alfanuméricos, ideal 8+.

### Tem lib pronta pra Kotlin/Android?

Sim, `bcprov-jdk18on` (BouncyCastle) funciona igual Java. Alternativa
mais leve: `at.favre.lib.crypto.HKDF` + adaptar pro scrypt via
`androidx.security.crypto`.

---

**Última atualização:** 2026-08-04
**Retaguarda Onix reference:** `server/_core/localAuth.ts::hashPassword`
**Contato dev retaguarda:** time Onix — abrir issue com tag `pdv-auth`
se precisar de vetor de teste ou clarificação.
