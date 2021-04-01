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
    l: ec_i,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "i")]
struct ec_i {
    i: Vec<String>,
}

impl ec {
    fn text(&self) -> String {
        let mut result = String::new();
        self.word.iter().for_each(|x| {
            result.push_str(
                format!(
                    "美[{}], 英[{}]\n\n",
                    x.usphone.as_deref().unwrap_or(""),
                    x.ukphone.as_deref().unwrap_or("")
                )
                .as_str(),
            );
            x.trs.iter().for_each(|y| {
                result.push_str(format!("- {}\n", &y.tr[0].l.i[0]).as_str());
            })
        });
        result
    }
}

impl rel_word {
    fn text(&self) -> String {
        let mut result = String::new();
        self.rels.iter().for_each(|i| {
            result.push_str(format!("- {}\n", &i.rel.pos).as_str());
            i.rel.words.iter().for_each(|x| {
                result.push_str(format!("  {} {}\n", &x.word, &x.tran).as_str());
            });
        });
        result
    }
}

impl phrs {
    fn text(&self) -> String {
        let mut result = String::new();
        self.phrs.iter().take(3).for_each(|one| {
            result.push_str(
                format!("- {} {}\n", &one.phr.headword.l.i, &one.phr.trs[0].tr.l.i).as_str(),
            );
        });
        result
    }
}

impl blng_sents_part {
    fn text(&self) -> String {
        let mut result = String::new();
        self.sentence_pair.iter().for_each(|one| {
            result.push_str(
                format!("- {}\n  {}\n", &one.sentence, &one.sentence_translation).as_str(),
            );
        });
        result
    }
}

impl FyResult {
    pub fn text(&self) -> String {
        format!(
            r#"{}
【同根】
{}
【短语】
{}
【例句】
{}"#,
            self.ec
                .as_ref()
                .map_or(ColoredString::from(""), |ec| ec.text().purple().bold()),
            self.rel_word
                .as_ref()
                .map_or(ColoredString::from(""), |rel| { rel.text().blue().bold() }),
            self.phrs
                .as_ref()
                .map_or(ColoredString::from(""), |phrs| phrs.text().red().bold()),
            self.blng_sents_part
                .as_ref()
                .map_or(ColoredString::from(""), |blng| blng.text().cyan().bold()),
        )
    }
}
