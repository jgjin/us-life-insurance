#![feature(option_filter)]

extern crate csv;

#[macro_use] extern crate serde_derive;

use csv::{Reader, Writer};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Record {
    rank: u8,
    type_: char,
    code: u32,
    name: String,
    premium: String,
    share: String,
}

#[derive(Debug)]
struct Insurers {
    insurers: HashMap<u32, Insurer>,
}

#[derive(Debug)]
struct Insurer {
    name: String,
    total_premiums: u64,
    states: Vec<StateRankShareSize>,
}

#[derive(Debug)]
struct StateRankShareSize {
    state: String,
    rank: u8,
    share: f32,
    size: u64,
}

#[derive(Debug)]
struct InsurerWithCode {
    insurer: Insurer,
    code: u32,
}

impl Insurers {
    fn add_record(
        &mut self,
        record: &mut Record,
        state: &str,
    ) {
        record.share.pop();
        let share = record.share.parse().expect("Error in parsing");
        let premium: u64 = record.premium.replace(",", "").parse().expect("Error in parsing");
        let insurer = self.insurers.entry(record.code).or_insert(
            Insurer {
                name: record.name.to_string(),
                total_premiums: 0u64,
                states: Vec::new(),
            }
        );
        insurer.total_premiums += premium;
        insurer.states.push(StateRankShareSize {
            state: state.to_string(),
            rank: record.rank,
            share: share,
            size: premium,
        });
    }
}

fn main() {
    // clean();
    
    let states = [
        "placeholder",
        "Alabama", "Alaska", "Arizona", "Arkansas", "California",
        "Colorado", "Connecticut", "Delaware", "District of Colombia", "Florida",
        "Georgia", "Hawaii", "Idaho", "Illinois", "Indiana",
        "Iowa", "Kansas", "Kentucky", "Louisiana", "Maine",
        "Maryland", "Massachusetts", "Michigan", "Minnesota", "Mississippi",
        "Missouri", "Montana", "Nebraska", "Nevada", "New Hampshire",
        "New Jersey", "New Mexico", "New York", "North Carolina", "North Dakota",
        "Ohio", "Oklahoma", "Oregon", "Pennsylvania", "Rhode Island",
        "South Carolina", "South Dakota", "Tennessee", "Texas", "Utah",
        "Vermont", "Virginia", "Washington", "West Virginia", "Wisconsin",
        "Wyoming",
    ];
    
    let mut insurers = Insurers {insurers: HashMap::new(),};
    let mut reader = Reader::from_path("total-clean.csv").expect("Error in file");
    let mut state = states.iter().peekable();
    reader.deserialize().map(|record_result| {
        let mut record: Record = record_result.expect("Error in record");
        if record.rank == 1 {
            state.next();
        }
        insurers.add_record(&mut record, state.peek().unwrap());
    }).last();

    let mut insurers_sorted: Vec<_> = insurers.insurers.drain().map(|tuple| {
        InsurerWithCode {
            insurer: tuple.1,
            code: tuple.0,
        }
    }).collect();
    insurers_sorted.sort_by(|a, b| {
        b.insurer.total_premiums.cmp(&a.insurer.total_premiums)
    });

    let mut writer1 = Writer::from_path("output-total.csv").expect("Error in file");
    writer1.write_record(&[
        "Name",
        "Total Amount",
        "Code",
    ]).unwrap();
    insurers_sorted.drain(..).map(|mut insurer_with_code| {
        let total_premiums = insurer_with_code.insurer.total_premiums;
        writer1.write_record(&[
            insurer_with_code.insurer.name,
            total_premiums.to_string(),
            insurer_with_code.code.to_string(),
        ]).expect("Error in writing");
        let mut writer2 = Writer::from_path(format!{
            "company-total/{}.csv",
            insurer_with_code.code,
        }).expect("Error in file");
        writer2.write_record(&[
            "State",
            "Rank within State",
            "Percentage of State",
            "Amount in State",
            "Amount in State as Percentage of Total Firm Amount",
        ]).unwrap();
        insurer_with_code.insurer.states.drain(..).map(|state_rank_share_size| {
            writer2.write_record(&[
                state_rank_share_size.state,
                state_rank_share_size.rank.to_string(),
                state_rank_share_size.share.to_string(),
                state_rank_share_size.size.to_string(),
                format!{
                    "{:.2}",
                    state_rank_share_size.size as f32 / total_premiums as f32 * 100f32,
                },
            ])
        }).last()
    }).last();
}

#[allow(dead_code)]
fn clean() {
    let mut reader = Reader::from_path("total.csv").expect("Error in file");
    let mut writer = Writer::from_path("total-clean.csv").expect("Error in file");

    reader.records().map(|record_result| {
        record_result.map(|record| {
            record.get(0).filter(|zeroth_field| {
                zeroth_field.parse::<u32>().is_ok() &&
                    record.get(1).unwrap() != ""
            }).map(|_| {
                writer.write_record(record.iter()).expect("Error in writing")
            })
        })
    }).last();
}
