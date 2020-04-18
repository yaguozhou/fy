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
    syno: Option<synos>,
    ec: Option<ec>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct synos {
    synos: Vec<syno>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct syno {
    syno: syno_one,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct syno_one {
    pos: String,
    tran: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ec {
    word: Vec<ec_word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "word")]
struct ec_word {
    usphone: Option<String>,
    ukphone: Option<String>,
    trs: Vec<ec_tr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "tr")]
struct ec_tr {
    tr: Vec<ec_l>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "l")]
struct ec_l {
    l: ec_i
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "i")]
struct ec_i {
    i: Vec<String>
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
            result.push_str(&(String::new() + &i.sentence + &"\n" + &i.sentence_translation + &"\n"));
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
                result.push_str(&(String::new() + &x.word + &" " + &x.tran + "\n"));
            }
            );
        }
        result
    }
}

impl synos {
    fn text(&self) -> String {
        let mut result = String::new();
        self.synos.iter().for_each(|one| {
            result.push_str(&(String::new() + &one.syno.pos + &" " + &one.syno.tran + &"\n"));
        });
        result
    }
}

impl ec {
    fn text(&self) -> String {
        let mut result = String::new();
        self.word.iter().for_each(|x| {
            result.push_str(&format!("美[{}], 英[{}]",
                                     &x.usphone.as_ref().unwrap_or(&"".to_string()),
                                     &x.ukphone.as_ref().unwrap_or(&"".to_string())));
            result.push_str("\n\n");
            x.trs.iter().for_each(|y| {
                result.push_str(&y.tr[0].l.i[0]);
                result.push_str("\n");
            })
        });
        result
    }
}

impl FyResult {
    pub fn text(&self) -> String {
        format!(r#"{}
{}
-----同根-----
{}
-----短语-----
{}
-----例句-----
{}"#,
                self.input.red().bold().underline(),
                match &self.ec {
                    Some(ec) => ec.text().red().bold(),
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
                    Some(blng) => blng.text().cyan().bold(),
                    _ => "".into()
                }
        )
    }
}