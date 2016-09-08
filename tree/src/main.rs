mod tree;
use tree::binary_tree::HuffmanCoder;


fn main() {

    let src = "asdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkfasdadljouiooijwerljlasdfjoiwuqwuerpoipqetu
    oqwienovnpiuqheruencuhquiehrquweroqwjeiqwoejrioqwfjciofkasdf
    asdhgasdjasdjfkhasdjkfancjkasnckjasdjkf";

    let mut coder = HuffmanCoder::new(src);
    let result = coder.ecode(src);
    println!("{:#?} = {:#?}", src, result);
    let decoded = coder.decode(result);
    println!("coder.decode(&result) = {:#?}", decoded);
    assert_eq!(src, decoded);



}




