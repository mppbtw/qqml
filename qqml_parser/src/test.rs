use crate::Question;
use crate::MultichoiceData;
use crate::MultichoiceAnswer;
use crate::parse;

#[test]
fn test_parse_multichoice_only() {
    let input = "ask multichoice (2) 'title' {
        * 'q1' (1);
        * 'q2' (1);
        * 'q3';
    } hints 'foo', 'bar';

    ask multichoice (1) 'othertitle' {
        * 'q1' (1) -> 'explanation';
        * 'q2' -> 'explanation 2';
    };";

    let mut q1data = MultichoiceData::new();
    q1data.set_text("title");
    q1data.set_max_marks(2);
    q1data.add_hint("foo");
    q1data.add_hint("bar");

    {
        let mut ans = MultichoiceAnswer::new();
        ans.set_text("q1");
        ans.set_marks(1);
        q1data.add_answer(ans);
    }
    {
        let mut ans = MultichoiceAnswer::new();
        ans.set_text("q2");
        ans.set_marks(1);
        q1data.add_answer(ans);
    }
    {
        let mut ans = MultichoiceAnswer::new();
        ans.set_text("q3");
        ans.set_marks(0);
        q1data.add_answer(ans);
    }

    let mut q2data = MultichoiceData::new();
    q2data.set_text("othertitle");
    q2data.set_max_marks(1);

    {
        let mut ans = MultichoiceAnswer::new();
        ans.set_text("q1");
        ans.set_marks(1);
        ans.set_explanation("explanation");
        q2data.add_answer(ans);
    }
    {
        let mut ans = MultichoiceAnswer::new();
        ans.set_text("q2");
        ans.set_marks(0);
        ans.set_explanation("explanation 2");
        q2data.add_answer(ans);
    }
    let expected = vec![
        Question::Multichoice(q1data),
        Question::Multichoice(q2data),
    ];

    let gotten = parse(input.to_owned()).unwrap();

    for (i, g) in gotten.iter().enumerate() {
        assert_eq!(g, expected.get(i).unwrap());
    }
}
