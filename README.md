# Sistema de Gerenciamento de Produtos e Cupons

API robusta para gestão de produtos, descontos e cupons promocionais com validações avançadas e controle transacional.

## ⚙️ Configuração

```bash
cd backend
cp .env.example .env

# Instalar Rust (se necessário) (ou usar o Docker):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Modo desenvolvimento:
cargo run

# Modo produção:
cargo run --release
```

# 🚀 Instruções para executar o ambiente Docker

## Passos para execução

```bash
# 1. Configure as variáveis de ambiente no arquivo .env
cp .env.example .env

# 2. Inicie os containers em modo destacado (detached)
docker-compose up --build -d
```

---

## Verificação dos containers rodando

```bash
podman ps -a
```

Exemplo de saída:

```
CONTAINER ID  IMAGE                                COMMAND               CREATED        STATUS        PORTS                   NAMES
6c8fc750944a  docker.io/bitnami/postgresql:latest  /opt/bitnami/scri...  3 seconds ago  Up 3 seconds  0.0.0.0:5432->5432/tcp  postgres
588fccaa3b81  localhost/server_app:latest          sh -c echo 'Start...  3 seconds ago  Created       0.0.0.0:3000->3000/tcp  server_app_1
```

---

## Iniciando manualmente o container da aplicação

```bash
podman start server_app_1
```

Saída esperada:

```
server_app_1
```

## Visualizando os logs do container da aplicação

```bash
podman logs -f server_app_1
```

Exemplo de log:

```
Starting application...

2025-06-26T14:52:16.687657Z  INFO isi_dev::interfaces::controller::product::get_products_controller: Start request

2025-06-26T14:52:16.687684Z  INFO isi_dev::domain::usecase::product::get_all_product_usecase: Start request

2025-06-26T14:52:16.701644Z  INFO isi_dev::domain::usecase::product::get_all_product_usecase: End request

2025-06-26T14:52:16.701659Z  INFO isi_dev::interfaces::controller::product::get_products_controller: End request
```

## 🌐 Endpoints da API

Base URL: `http://localhost:3000/api/v1`

### 1. Produtos

#### Listar produtos com filtros

```bash
curl -X GET "http://localhost:3000/api/v1/products?page=1&limit=5&search=cafe&min_price=10&max_price=100&has_discount=true&sort_by=name&sort_order=asc"
```

#### Criar produto

```bash
curl -X POST "http://localhost:3000/api/v1/products" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Café Premium",
    "description": "100% arábica",
    "stock": 250,
    "price": 2590
  }'
```

#### Atualizar produto (PATCH)

```bash
curl -X PATCH "http://localhost:3000/api/v1/products/550e8400-e29b-41d4-a716-446655440000" \
  -H "Content-Type: application/json-patch+json" \
  -d '[
    { "op": "replace", "path": "/name", "value": "Café Gourmet" },
    { "op": "replace", "path": "/stock", "value": 50 }
  ]'
```

#### Aplicar cupom

```bash
curl -X POST "http://localhost:3000/api/v1/products/550e8400-e29b-41d4-a716-446655440000/discount/coupon" \
  -H "Content-Type: application/json" \
  -d '{"code": "PROMO20"}'
```

### 2. Cupons

#### Criar cupom

```bash
curl -X POST "http://localhost:3000/api/v1/coupons" \
  -H "Content-Type: application/json" \
  -d '{
    "code": "PROMO20",
    "type": "percent",
    "value": 20.0,
    "one_shot": true,
    "valid_from": "2025-01-01T00:00:00Z",
    "valid_until": "2025-12-31T23:59:59Z",
    "max_uses": 100
  }'
```

#### Buscar cupom

```bash
curl -X GET "http://localhost:3000/api/v1/coupons/PROMO20"
```

---

## 📌 Regras de Negócio Importantes

### 🚫 Validações de Produtos

1. **Nome único**: Não pode repetir após normalização (remove espaços extras/acentos)
2. **Estoque**: Entre 0 e 999.999 unidades
3. **Preço**:
   - Mínimo: R$ 0.01
   - Máximo: R$ 1.000.000,00
4. **Datas**: `created_at` sempre no passado/presente (nunca futuro)

### 🎟 Validações de Cupons

| Campo         | Regras                               |
| ------------- | ------------------------------------ |
| `code`        | 4-20 caracteres, único, sem símbolos |
| `type`        | `fixed` ou `percent`                 |
| `value`       | Percentual: 1-80%, Fixo: > R$ 0,00   |
| `valid_until` | Máximo 5 anos após `valid_from`      |

### ⚠️ Regras de Descontos

- **1 desconto por produto**: Tentativas adicionais retornam erro 409
- **Preço mínimo**: Valor final nunca abaixo de R$ 0,01
- **Cupons expirados**: Rejeitados com erro 400
- **Cálculo dinâmico**: Preço original nunca alterado no banco

---

## 📊 Respostas de Exemplo

### Produto com desconto (200 OK)

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Café Premium",
  "price": 2590,
  "final_price": 2072,
  "discount": {
    "type": "percent",
    "value": 20,
    "applied_at": "2025-06-20T12:30:45Z"
  }
}
```

### Erro de validação (400 Bad Request)

```json
{
  "status": 400,
  "message": "Validation error: value: Must be between 0.01 and 1000000.00"
}
```

### Conflito de desconto (409 Conflict)

```json
{
  "status": 409,
  "message": "Product already has an active discount"
}
```

### Preço inválido (422 Unprocessable Entity)

```json
{
  "status": 422,
  "message": "Final price would be invalid (less than 0.01)"
}
```

---

## 🔐 Códigos de Status

| Código | Situação                             |
| ------ | ------------------------------------ |
| 200    | OK - Requisição bem-sucedida         |
| 201    | Created - Recurso criado             |
| 204    | No Content - Ação sem retorno        |
| 400    | Bad Request - Dados inválidos        |
| 404    | Not Found - Recurso inexistente      |
| 409    | Conflict - Estado inconsistente      |
| 412    | Precondition Failed - ETag inválido  |
| 422    | Unprocessable Entity - Regra violada |

---

Vou adicionar uma seção dedicada aos testes de integração no README, incluindo instruções detalhadas e exemplos de execução. Aqui está a versão atualizada:

````markdown
# Sistema de Gerenciamento de Produtos e Cupons

## 🧪 Testes de Integração

O sistema inclui testes de integração abrangentes que validam as regras de negócio e a interação com o banco de dados.

### Executando os Testes

```bash
cd backend

# Criar banco de testes (primeira execução)
docker-compose -f docker-compose.test.yml up --build -d

# Executar testes
cargo test -- --test-threads=1

# Parar banco de testes após execução
docker-compose -f docker-compose.test.yml down
```

### Configuração do Ambiente de Testes

```yaml
# docker-compose.test.yml
version: "3.8"
services:
  postgres-test:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: test_user
      POSTGRES_PASSWORD: test_password
      POSTGRES_DB: test_db
    ports:
      - "5433:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U test_user"]
      interval: 5s
      timeout: 5s
      retries: 10
```

### Estrutura de Testes

```
backend
├── src
└── tests
  └── integration_tests
     ├── coupon_tests
     │   ├── create_coupon_test.rs
     │   ├── delete_coupon_test.rs
     │   ├── get_coupon_test.rs
     │   ├── mod.rs
     │   └── update_coupon_test.rs
     ├── mod.rs
     └── product_tests
     ├── create_product_test.rs
     ├── delete_product_test.rs
     ├── get_product_test.rs
     ├── health_check_test.rs
     ├── mod.rs
     └── update_product_test.rs
```

### Cobertura de Testes

| Funcionalidade          | Casos Testados                            |
| ----------------------- | ----------------------------------------- |
| Criação de produtos     | Validações, nomes duplicados, estoque     |
| Aplicação de cupons     | Conflitos, cálculo, validações            |
| Atualização de produtos | Operações PATCH, controle de concorrência |
| Validações de cupons    | Tipos, datas, limites de uso              |
| Regras de desconto      | Preço mínimo, sobreposição                |

### Saída de Exemplo

```bash
running 27 tests
test tests::products::create_product_invalid_stock ... ok
test tests::coupons::apply_expired_coupon ... ok
test tests::discounts::discount_conflict ... ok
...
test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured
```

### Observações sobre Testes

1. Banco de dados isolado (porta 5433)
2. Transações atômicas (cada teste roda em transação separada)
3. Paralelismo controlado (`--test-threads=1`)
4. Setup automático de fixtures
5. Validação de status HTTP e corpo das respostas
````

O comando `cargo test -- --test-threads=1` é essencial para evitar conflitos entre testes que acessam o mesmo banco de dados simultaneamente.

## ⚠️ Observações Críticas

1. **Identificadores**:

   - Produtos: UUID
   - Cupons: Código imutável (ex: "PROMO20")

2. **Formatação**:

   - Datas: ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`)
   - Valores monetários: 2 casas decimais (ex: 25.90)

3. **Operações Especiais**:

   - PATCH usa [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902)
   - DELETE sempre soft delete (inativação)
   - Restauração via `POST /products/{id}/restore`

4. **Concorrência**:

   - Controle via ETag em operações críticas
   - Validação obrigatória em updates (PATCH)

5. **Paginação**:
   - Parâmetros: `page`, `limit` (max 50)
   - Metadados incluídos na resposta:
   ```json
   "meta": {
     "page": 1,
     "limit": 10,
     "total_items": 85,
     "total_pages": 9
   }
   ```
6. **Testes**:
   - Testes de integração com banco de dados
