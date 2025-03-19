mod casino;
mod hash;
mod merkle;
mod mul;
mod square_root;

fn main() {
    mul::multiplication();
    casino::casino();
    square_root::square_root();

    // hash::hash_circuit();
    merkle::merke_nohash0();
    merkle::merke_nohash1();
    // merkle::merke_nohash3();
    // merkle::merke_nohash4();
    // merkle::merke_nohash5();
    // merkle::merke_circuit_with_hash();
}
