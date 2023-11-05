use algorithm::data_structures::BinaryIndexedTree;

#[test]
fn query_one() {
    let vec = vec![2; 20];
    let t = BinaryIndexedTree::new(vec);
    assert!(t.query_one(0) == 2);
    assert!(t.query_one(9) == 20);
    assert!(t.query_one(14) == 30);
    assert!(t.query_one(19) == 40);
}

#[test]
fn query_range() {
    let vec = vec![2; 20];
    let t = BinaryIndexedTree::new(vec);
    assert!(t.query_range(0, 0) == 0);
    assert!(t.query_range(0, 5) == 10);
    assert!(t.query_range(5, 10) == 10);
    assert!(t.query_range(10, 19) == 18);
}

#[test]
fn query_edit() {
    let vec = vec![2; 20];
    let mut t = BinaryIndexedTree::new(vec);
    t.edit(0, 2);
    assert!(t.query_one(0) == 4);
    t.edit(0, -2);
    assert!(t.query_one(9) == 20);
    t.edit(5, 3);
    assert!(t.query_one(14) == 33);
    t.edit(10, 10);
    assert!(t.query_one(19) == 53);
}
