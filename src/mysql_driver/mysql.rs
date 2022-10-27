pub mod sql_query {
    use mysql::*;
    use mysql::prelude::Queryable;
    use crate::structs::items::Items as Items;
    const DB_URL: &str = "mysql://root:@localhost:3306/shopdb";

    pub fn connect() -> Pool {
        let pool = mysql::Pool::new(DB_URL).unwrap();
        pool
    }

    pub fn get_items() -> Vec<Items> {
        let mut table: Vec<Items> = Vec::new();
        let pool = connect();
        let mut conn = pool.get_conn().unwrap();
        //get the rows from the table
        conn.query_iter("select id, quantity, product_name from shopdb.items")
            .unwrap()
            .for_each(|r|{
                let row:(i64,i64, String) = from_row(r.unwrap());
                let item: Items = Items::new(
                    row.0,
                    row.1,
                    row.2
                );
                table.push(item)
            });
        table
    }

    #[cfg(test)]
    mod test_sql {
        use mysql::*;
        use crate::structs::items::Items as Items;
        use super::connect as test_connect;
        use super::get_items as test_get_items;

        #[test]
        #[ignore = "not yet implemented"]
        fn test_connection() {
            let _pool_test = test_connect();
        }

        #[test]
        fn test_get_db_items() {
            //real behavior
            let table = test_get_items();

            //call db to get excpected behavior
            let mut table_sql: Vec<Items> = Vec::new();
            let table_sql = [
                Items::new(1,1000,String::from("Pokeballs")),
                Items::new(2,1000,String::from("Superballs")),
                Items::new(3,1000,String::from("Ultraballs")),
                Items::new(4,1000,String::from("Honorballs")),
                Items::new(5,1000,String::from("Potions")),
                Items::new(6,1000,String::from("Super Potions")),
                Items::new(7,1000,String::from("Hiper Potions")),
                Items::new(8,1000,String::from("Max Potions")),
                Items::new(9,1000,String::from("Revive")),
                Items::new(10,1000,String::from("Max Revive"))
            ];
            let mut iter: usize = 0;
            for elem in &table{
                assert_eq!(elem.get_id(),table_sql[iter].get_id());
                iter = iter + 1;
            }
        }
    }
}