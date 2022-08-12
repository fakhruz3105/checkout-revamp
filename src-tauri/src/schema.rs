table! {
    borrowed_payments (id) {
        id -> Nullable<Integer>,
        amount -> Double,
        borrower_id -> Nullable<Integer>,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    borrowers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    financial_statements (id) {
        id -> Nullable<Integer>,
        value -> Double,
        user_id -> Integer,
        description -> Text,
        metadata -> Nullable<Text>,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    items (id) {
        id -> Nullable<Integer>,
        barcode -> Double,
        name -> Text,
        stock -> Integer,
        sold -> Integer,
        sell_price -> Double,
        buy_price -> Nullable<Double>,
        profit -> Double,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    receipts (id) {
        id -> Nullable<Integer>,
        readable_id -> Text,
        user_id -> Integer,
        borrower_id -> Nullable<Integer>,
        items -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

joinable!(borrowed_payments -> borrowers (borrower_id));
joinable!(financial_statements -> users (user_id));
joinable!(receipts -> borrowers (borrower_id));
joinable!(receipts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    borrowed_payments,
    borrowers,
    financial_statements,
    items,
    receipts,
    users,
);
