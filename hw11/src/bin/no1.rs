fn make_document(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|x| x.to_string())
        .collect()
}

fn rank_documents(docs: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut documents = docs.clone();
    let n = documents.len();
    for _i in 0..n {
        for _j in 0..n-_i-1 {
            if documents[_j].len() < documents[_j + 1].len() {
                documents.swap(_j, _j + 1);
            }
        }
    }
    documents
}

fn main() {
    let text1 = "I am at home.";
    let text2 = "a\n\nab\n\nb";
    let text3 = "\
        Today is Friday the 13th.\n\
        The bustle in a house.\n\
        \n\
        The sweeping up the heart.\n\
        And putting love away.\n\
        ";

    let paragraph = make_document(text2);
    println!("{:?}",paragraph);

    let docs1 = make_document(text1); //one paragraph
    let docs2 = make_document(text2); //three paragraphs
    let docs3 = make_document(text3); //two paragraphs
    let unranked = vec![docs1.clone(), docs2.clone(), docs3.clone()];
    println!("Unranked: ");
    println!("{:?}", unranked);

    let ranked = rank_documents(&unranked);
    println!("Ranked: ");
    println!("{:?}", ranked);
}

#[test]
fn test_rank_documents() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";

    let empty_doc = Vec::new();
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![empty_doc.clone(), doc1.clone(), doc3.clone(), doc2.clone()];
    let ranked = rank_documents(&docs);
    assert_eq!(ranked, [doc3, doc2, doc1, empty_doc]);
}