use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::env::Env;

#[derive(Parser)]
#[grammar = "dql.pest"]
struct DqlParser;

fn parse_pair(pair: Pair<Rule>, env: &mut Env) -> String {
    match pair.as_rule() {
        Rule::selectorFragment | Rule::htmlName | Rule::string | Rule::number => {
            String::from(pair.as_str())
        }
        Rule::selectAll => format!(" {}", pair.as_str()),
        Rule::functionCall => {
            let mut iter = pair.into_inner();
            let name = iter.next().unwrap().as_str();
            let args = iter
                .next()
                .map(|p| parse_pair(p, env))
                .unwrap_or("".to_owned());
            return if args.len() > 0 {
                format!(":{}({})", name, args)
            } else {
                format!(":{}", name)
            };
        }
        Rule::logicFragment => format!("[{}]", pair.as_str()),
        Rule::selectRaw => pair.into_inner().next().unwrap().as_str().replace("\"", ""),
        Rule::cte => {
            let mut iter = pair.into_inner();
            let name = parse_pair(iter.next().unwrap(), env);
            let expression = parse_pair(iter.next().unwrap(), env);
            env.insert_cte(name, expression);
            String::new()
        }
        Rule::selectExpression => {
            let mut iter = pair.into_inner();
            let first = iter.next().unwrap();
            let second = iter.next();
            if first.as_rule() == Rule::prefix {
                format!("{} {}", first.as_str(), second.unwrap().as_str())
            } else {
                String::from(first.as_str())
            }
        }
        Rule::rawStatement => {
            let mut iter = pair.into_inner();
            let select = parse_pair(iter.next().unwrap(), env);
            let from = iter.next().map(|p| p.as_str()).unwrap_or("");
            let from_body = env
                .get_cte(from)
                .map(|s| s.clone())
                .unwrap_or(String::new());
            format!("{} {}", from_body, select)
        }
        Rule::unionStatement => pair
            .into_inner()
            .map(|p| {
                let name = p.as_str();
                let value = env.get_cte(name);
                value.map(|s| s.trim()).unwrap_or("")
            })
            .filter(|s| s.len() > 0)
            .collect::<Vec<_>>()
            .join(", "),
        Rule::normalStatement => {
            let mut iter = pair.into_inner();
            let select = parse_pair(iter.next().unwrap(), env);
            let from = iter.find_first_tagged("from").unwrap().as_str();
            let from_body = env
                .get_cte(from)
                .map(|s| s.clone())
                .unwrap_or(String::new());
            let where_clause = iter
                .find_first_tagged("where")
                .map(|p| parse_pair(p, env))
                .unwrap_or(String::new());
            format!("{} {}{}", from_body, select.as_str(), where_clause.as_str())
        }
        _ => pair.into_inner().map(|p| parse_pair(p, env)).collect(),
    }
}

pub fn parse_dql(code: &str) -> String {
    let result = DqlParser::parse(Rule::dql, code);

    let mut env = Env::new();
    env.insert_cte(String::from("body"), String::new());

    match result {
        Ok(mut p) => parse_pair(p.next().unwrap(), &mut env).trim().to_string(),
        Err(e) => {
            dbg!(e);
            return String::from("");
        }
    }
}
