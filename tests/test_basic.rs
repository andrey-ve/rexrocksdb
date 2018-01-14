extern crate rocksdb;

use rocksdb::{DB, Options, Error, DBVector};

fn setup_db() -> Result<DB, Error> {
    let path = "/Users/andrey/tmp/rocksdb-test-basic/";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_num_levels(2);
    DB::open(&opts, path)
}

fn cleanup(db: DB) {
    drop(db);
}

#[test]
fn test_no_val4key() {
    let db = setup_db().unwrap();

    match db.get(b"k1") {
        Ok(opt) => assert!(opt.is_none()),
        _ => assert!(false),
    }

    cleanup(db);
}

#[test]
fn test_val4key() {
    let db = setup_db().unwrap();

    assert!(db.put(b"k1", b"v1").is_ok());

    match db.get(b"k1") {
        Ok(opt) => {
            match opt {
                Some(val) => assert_eq!(val.to_utf8().unwrap(), "v1"),
                None => assert!(false),
            }
        }
        _ => assert!(false),
    }

    // cleanup
    assert!(db.delete(b"k1").is_ok());
    cleanup(db);
}

fn _read_key(get_res: Result<Option<DBVector>, Error>, key: &[u8], msg: &str) -> String {
    assert!(get_res.is_ok());
    let opt = get_res.unwrap();
    assert!(opt.is_some());
    let val_str= opt.unwrap().to_utf8().unwrap().to_string();
    println!("{} key:{} = {}", msg,
             std::str::from_utf8(key).unwrap(), val_str);
    val_str
}

#[test]
fn test_val_rw() {
    let db = setup_db().unwrap();

    assert!(db.put(b"k1", b"v1").is_ok());

    let val_str = _read_key(db.get(b"k1"), b"k1", "First val for ");
    assert_eq!(val_str, "v1");


    assert!(db.put(b"k1", b"v2").is_ok());
    let val_str = _read_key(db.get(b"k1"), b"k1", "First val for ");
    assert_eq!(val_str, "v2");

    // cleanup
    assert!(db.delete(b"k1").is_ok());
    cleanup(db);
}

#[test]
fn test_val_rw_ver() {
    let db = setup_db().unwrap();

    assert!(db.put(b"k1", b"v1").is_ok());

    let val_str = _read_key(db.get(b"k1"), b"k1", "First val for ");
    assert_eq!(val_str, "v1");

    {
        let snap = db.snapshot();
        let val_str = _read_key(snap.get(&db,b"k1"), b"k1",
                                "From snapshot: First val for ");
        assert_eq!(val_str, "v1");

        assert!(db.put(b"k1", b"v2").is_ok());
        let val_str = _read_key(db.get(b"k1"), b"k1", "Second val for ");
        assert_eq!(val_str, "v2");

        let val_str = _read_key(snap.get(&db,b"k1"), b"k1",
                                "From snapshot: Second val for ");
        assert_eq!(val_str, "v1");
    }

    // cleanup
    assert!(db.delete(b"k1").is_ok());
    cleanup(db);
}

