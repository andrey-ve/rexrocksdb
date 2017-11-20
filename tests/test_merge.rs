extern crate rocksdb;

use rocksdb::{DB, Options, Error, DBVector,
              MergeOperands};

fn setup_db_opt(mut opts: Options) -> Result<DB, Error> {
    let path = "/Users/andrey/tmp/rocksdb-test-basic/";
    opts.create_if_missing(true);
    opts.set_num_levels(2);
    DB::open(&opts, path)
}

fn cleanup(db: DB) {
    drop(db);
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

#[allow(unused_variables)]
fn _concat_merge(
    new_key: &[u8],
    existing_val: Option<&[u8]>,
    operands: &mut MergeOperands,
) -> Vec<u8> {
    let nops = operands.size_hint().0;
    let mut result: Vec<u8> = Vec::with_capacity(nops);
    if let Some(v) = existing_val {
        println!("Some::_concat_merge: vlen = {}", v.len());
        for e in v {
            result.push(*e);
        }
    } else {
        println!("None::_concat_merge");

    }
    for op in operands {
        for e in op {
            result.push(*e);
        }
    }
    result
}

#[test]
fn test_concat_merge() {

    let mut opts = Options::default();
    opts.set_merge_operator("test cancat operator", _concat_merge);
    let db = setup_db_opt(opts).unwrap();

    // create merged value
    assert!(db.put(b"k1", b"a").is_ok());
    assert!(db.merge(b"k1", b"b").is_ok());
    assert!(db.merge(b"k1", b"c").is_ok());
    assert!(db.merge(b"k1", b"d").is_ok());
    assert!(db.merge(b"k1", b"efg").is_ok());
    assert!(db.merge(b"k1", b"h").is_ok());

    let val_str = _read_key(db.get(b"k1"), b"k1", "Merged val for ");
    assert!(db.delete(b"k1").is_ok());
    assert_eq!(val_str, "abcdefgh");

    // check merged value removed with the key
    let d = db.get(b"k1");
    assert!(d.is_ok());
    assert!(d.unwrap().is_none());

    cleanup(db);
}

#[test]
fn test_concat_merge_no_put() {

    let mut opts = Options::default();
    opts.set_merge_operator("test cancat operator", _concat_merge);
    let db = setup_db_opt(opts).unwrap();

    // create merged value
    //assert!(db.put(b"k1", b"a").is_ok());
    assert!(db.merge(b"k1", b"a").is_ok());
    assert!(db.merge(b"k1", b"b").is_ok());
    assert!(db.merge(b"k1", b"c").is_ok());
    assert!(db.merge(b"k1", b"d").is_ok());
    assert!(db.merge(b"k1", b"efg").is_ok());
    assert!(db.merge(b"k1", b"h").is_ok());

    let val_str = _read_key(db.get(b"k1"), b"k1", "Merged val for ");
    assert!(db.delete(b"k1").is_ok());
    assert_eq!(val_str, "abcdefgh");

    // check merged value removed with the key
    let d = db.get(b"k1");
    assert!(d.is_ok());
    assert!(d.unwrap().is_none());

    cleanup(db);
}
