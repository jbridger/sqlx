use sqlx::{postgres::PgConnection, Connection as _, Row};

async fn connect() -> anyhow::Result<PgConnection> {
    Ok(PgConnection::open(dotenv::var("DATABASE_URL")?).await?)
}

macro_rules! test {
    ($name:ident: $ty:ty: $($text:literal == $value:expr),+) => {
        #[async_std::test]
        async fn $name () -> anyhow::Result<()> {
            let mut conn = connect().await?;

            $(
                let row = sqlx::query(&format!("SELECT {} = $1, $1 as _1", $text))
                    .bind($value)
                    .fetch_one(&mut conn)
                    .await?;

                assert!(row.get::<bool, _>(0));
                assert!($value == row.get::<$ty, _>("_1"));
            )+

            Ok(())
        }
    }
}

test!(postgres_bool: bool: "false::boolean" == false, "true::boolean" == true);

test!(postgres_smallint: i16: "821::smallint" == 821_i16);
test!(postgres_int: i32: "94101::int" == 94101_i32);
test!(postgres_bigint: i64: "9358295312::bigint" == 9358295312_i64);

test!(postgres_real: f32: "9419.122::real" == 9419.122_f32);
test!(postgres_double: f64: "939399419.1225182::double precision" == 939399419.1225182_f64);

test!(postgres_text: String: "'this is foo'" == "this is foo", "''" == "");
