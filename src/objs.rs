/*
 * Copyright (c) 2020 Yaguo Zhou
 * fy is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

use colored::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FyResult {
    input: String,
    blng_sents_part: Option<blng_sents_part>,
    phrs: Option<phrs>,
    rel_word: Option<rel_word>,
    simple: Option<simple>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct simple {
    word: Vec<phone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct phone {
    usphone: Option<String>,
    ukphone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct blng_sents_part {
    #[serde(alias = "sentence-pair")]
    sentence_pair: Vec<sen>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct sen {
    sentence: String,
    #[serde(alias = "sentence-translation")]
    sentence_translation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct phrs {
    phrs: Vec<phr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct phr {
    phr: phr_one,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct phr_one {
    headword: headword,
    trs: Vec<tr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct headword {
    l: l,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct l {
    i: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct tr {
    tr: headword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct rel_word {
    rels: Vec<rel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct rel {
    rel: rel_one,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct rel_one {
    pos: String,
    words: Vec<word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct word {
    word: String,
    tran: String,
}

impl phrs {
    fn text(&self) -> String {
        let mut result = String::new();
        for one in &self.phrs {
            let eng = &one.phr.headword.l.i;
            let chi = &one.phr.trs[0].tr.l.i;
            result.push_str(&eng);
            result.push_str(" ");
            result.push_str(&chi);
            result.push_str("\n");
        }
        result
    }
}

impl blng_sents_part {
    fn text(&self) -> String {
        let mut result = String::new();
        for i in &self.sentence_pair {
            result.push_str(&(i.sentence));
            result.push_str("\n");
            result.push_str(&(i.sentence_translation));
            result.push_str("\n");
        }
        result
    }
}

impl rel_word {
    fn text(&self) -> String {
        let mut result = String::new();
        for i in &self.rels {
            let pos = &i.rel.pos;
            result.push_str(&pos);
            result.push_str("\n");
            i.rel.words.iter().for_each(|x| {
                let mut s = String::new();
                s.push_str(&x.word);
                s.push_str(" ");
                s.push_str(&x.tran);
                result.push_str(&s);
                result.push_str("\n");
            }
            );
        }
        result
    }
}

impl FyResult {
    pub fn text(&self) -> String {
        format!(r#"{}
美[{}], 英[{}]

{}
-----短语-----
{}
-----例句-----
{}"#,
                self.input.red().bold(),
                match &self.simple {
                    Some(simple) => {
                        simple.word[0].usphone.as_ref().unwrap_or(&"".to_string()).green().bold()
                    }
                    _ => "".into()
                },
                match &self.simple {
                    Some(simple) => {
                        simple.word[0].ukphone.as_ref().unwrap_or(&"".to_string()).green().bold()
                    }
                    _ => "".into()
                },
                match &self.rel_word {
                    Some(rel) => rel.text().blue().bold(),
                    _ => "".into()
                },
                match &self.phrs {
                    Some(phrs) => phrs.text().purple().bold(),
                    _ => "".into()
                },
                match &self.blng_sents_part {
                    Some(blng) => blng.text().red().bold(),
                    _ => "".into()
                }
        )
    }
}