Clean Architecture Project
│ src
│ ├── application
│ │   ├── mod.rs
│ │   ├── repository
│ │   │   ├── coupon_in_memory_repository.rs
│ │   │   ├── coupon_postgres_repository.rs
│ │   │   ├── discount_in_memory_repository.rs
│ │   │   ├── discount_postgres_repository.rs
│ │   │   ├── mod.rs
│ │   │   ├── product_in_memory_repository.rs
│ │   │   └── product_postgres_repository.rs
│ │   └── usecase
│ │   ├── mod.rs
│ │   └── patch_operation.rs
│ ├── domain
│ │   ├── entity
│ │   │   ├── coupon_entity.rs
│ │   │   ├── discount_entity.rs
│ │   │   ├── mod.rs
│ │   │   └── product_entity.rs
│ │   ├── mod.rs
│ │   ├── repository
│ │   │   ├── coupon_repository.rs
│ │   │   ├── discount_repository.rs
│ │   │   ├── mod.rs
│ │   │   └── product_repository.rs
│ │   ├── usecase
│ │   │   ├── coupon
│ │   │   │   ├── create_coupon_usecase.rs
│ │   │   │   ├── delete_coupon_usecase.rs
│ │   │   │   ├── get_coupons_usecase.rs
│ │   │   │   ├── get_coupon_usecase.rs
│ │   │   │   ├── mod.rs
│ │   │   │   └── update_coupon_usecase.rs
│ │   │   ├── discount
│ │   │   │   ├── apply_coupon_discount_usecase.rs
│ │   │   │   ├── apply_percent_discount_usecase.rs
│ │   │   │   ├── mod.rs
│ │   │   │   └── remove_discount_usecase.rs
│ │   │   ├── mod.rs
│ │   │   └── product
│ │   │   ├── create_product_usecase.rs
│ │   │   ├── delete_product_usecase.rs
│ │   │   ├── get_all_product_usecase.rs
│ │   │   ├── get_product_usecase.rs
│ │   │   ├── mod.rs
│ │   │   ├── restore_product_usecase.rs
│ │   │   └── update_product_usecase.rs
│ │   └── utils
│ │   ├── coupon_value_validate.rs
│ │   ├── mod.rs
│ │   ├── normalize_name.rs
│ │   └── statics.rs
│ ├── frameworks
│ │   ├── adapter
│ │   │   ├── axum.rs
│ │   │   └── mod.rs
│ │   ├── axum
│ │   │   ├── handler
│ │   │   │   ├── coupon
│ │   │   │   │   ├── create_coupon.rs
│ │   │   │   │   ├── delete_coupon_by_code.rs
│ │   │   │   │   ├── get_coupon_by_code.rs
│ │   │   │   │   ├── get_coupons.rs
│ │   │   │   │   ├── mod.rs
│ │   │   │   │   └── update_coupon_by_code.rs
│ │   │   │   ├── discount
│ │   │   │   │   ├── apply_coupon_discount.rs
│ │   │   │   │   ├── apply_percent_discount.rs
│ │   │   │   │   ├── mod.rs
│ │   │   │   │   └── remove_discount_active.rs
│ │   │   │   ├── mod.rs
│ │   │   │   └── product
│ │   │   │   ├── create_product.rs
│ │   │   │   ├── delete_product_by_id.rs
│ │   │   │   ├── get_product_by_id.rs
│ │   │   │   ├── get_products.rs
│ │   │   │   ├── mod.rs
│ │   │   │   ├── restore_product_by_id.rs
│ │   │   │   └── update_product_by_id.rs
│ │   │   ├── mod.rs
│ │   │   └── server.rs
│ │   ├── mod.rs
│ │   └── sqlx
│ │   ├── mod.rs
│ │   ├── pool.rs
│ │   └── run_schema.rs
│ ├── interfaces
│ │   ├── controller
│ │   │   ├── coupon
│ │   │   │   ├── create_coupon_controller.rs
│ │   │   │   ├── delete_coupon_controller.rs
│ │   │   │   ├── get_coupon_controller.rs
│ │   │   │   ├── get_coupons_controller.rs
│ │   │   │   ├── mod.rs
│ │   │   │   └── update_coupon_controller.rs
│ │   │   ├── discount
│ │   │   │   ├── apply_coupon_discount_controller.rs
│ │   │   │   ├── apply_percent_discount_controller.rs
│ │   │   │   ├── mod.rs
│ │   │   │   └── remove_discount_controller.rs
│ │   │   ├── mod.rs
│ │   │   └── product
│ │   │   ├── create_product_controller.rs
│ │   │   ├── delete_product_controller.rs
│ │   │   ├── get_product_controller.rs
│ │   │   ├── get_products_controller.rs
│ │   │   ├── mod.rs
│ │   │   ├── restore_product_controller.rs
│ │   │   └── update_product_controller.rs
│ │   ├── handler
│ │   │   ├── generic_handler.rs
│ │   │   └── mod.rs
│ │   └── mod.rs
│ ├── lib.rs
│ └── main.rs
└── tests
├── integration_tests
│   ├── coupon_tests
│   │   ├── create_coupon_test.rs
│   │   ├── delete_coupon_test.rs
│   │   ├── get_coupon_test.rs
│   │   ├── mod.rs
│   │   └── update_coupon_test.rs
│   ├── mod.rs
│   └── product_tests
│   ├── create_product_test.rs
│   ├── delete_product_test.rs
│   ├── get_product_test.rs
│   ├── health_check_test.rs
│   ├── mod.rs
│   └── update_product_test.rs
├── lib.rs
└── utils
├── mod.rs
└── start_server.rs
