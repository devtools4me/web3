use polars::prelude::*;
use chrono::prelude::*;
use eyre::Result;

pub async fn mom() -> Result<()> {
    let df1 = CsvReader::from_path("data/MSFT-1m.csv").unwrap().finish().unwrap();
    println!("{}", df1);

    let df2: DataFrame = df1.describe(None).unwrap();
    println!("{}", df2);

    println!("{}", df1.head(None));
    println!("{}", df1.tail(None));

    let subset = &df1[..2];
    println!("{:?}", subset);

    let open = df1.column("Open");
    println!("{:?}", open);

    println!("{:?}", df1.null_count());

    Ok(())
}

// pub fn timestamp_to_local_date(timestamp_milis: i64) -> Date<Local> {
//     let naive = NaiveDateTime::from_timestamp(timestamp_milis / 1000, 0);
//     Local.from_utc_datetime(&naive).date()
// }
//
// pub fn str_to_local_date(s: &str) -> Date<Local> {
//     let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d").unwrap();
//     Local.from_utc_datetime(&naive).date()
// }