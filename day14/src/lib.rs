use std::collections::HashMap;

pub struct Reaction {
    // element, (generated, need => (nb, el))
    pub relation: HashMap<String, (u64, Vec<(u64, String)>)>,
    pub remaining: HashMap<String, u64>,
    pub used: HashMap<String, u64>,
}

impl Reaction {
    pub fn generate(&mut self, nb: u64, el: String) {
        let have = self.remaining.entry(el.clone()).or_insert(0);
        if *have >= nb {
            return;
        }
        if &el == "ORE" {
            let ore = self.remaining.entry(String::from("ORE")).or_insert(0);
            *ore += nb - *ore;
            let used = self.used.entry(String::from("ORE")).or_insert(0);
            *used += nb - *ore;
            return;
        }
        let (generated, need) = self.relation.get(&el).unwrap().clone();
        for el in need {
            self.generate(el.0, el.1.clone());
            let need = self.remaining.entry(el.1.clone()).or_insert(0);
            *need -= el.0;
            let used = self.used.entry(el.1).or_insert(0);
            *used += el.0;
        }
        let have = self.remaining.get_mut(&el).unwrap();
        *have += generated;
        if nb > *have {
            self.generate(nb, el);
        }
    }

    pub fn try_generate(&mut self, nb: u64, el: String) {
        let have = self.remaining.entry(el.clone()).or_insert(0);
        if *have >= nb {
            return;
        }
        if &el == "ORE" {
            let ore = self.remaining.entry(String::from("ORE")).or_insert(0);
            *ore += nb - *ore;
            let used = self.used.entry(String::from("ORE")).or_insert(0);
            *used += nb - *ore;
            return;
        }
        let (generated, need) = self.relation.get(&el).unwrap().clone();
        for el in need {
            self.generate(el.0, el.1.clone());
            let need = self.remaining.entry(el.1.clone()).or_insert(0);
            *need -= el.0;
            let used = self.used.entry(el.1).or_insert(0);
            *used += el.0;
        }
        let have = self.remaining.get_mut(&el).unwrap();
        *have += generated;
        if nb > *have {
            self.generate(nb, el);
        }
    }

    pub fn used(&self, s: &str) -> u64 {
        *self.used.get(s).unwrap_or(&0)
    }
}

impl std::str::FromStr for Reaction {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut relation = HashMap::new();

        for line in input.lines() {
            let mut line = line.split("=>");
            let (need, result) = (line.next().unwrap(), line.next().unwrap());

            let need = need
                .split(',')
                .map(|el| {
                    let mut el = el.trim().split(' ');
                    (
                        el.next().unwrap().trim().parse::<u64>().unwrap(),
                        String::from(el.next().unwrap().trim()),
                    )
                })
                .collect();

            let mut result = result.trim().split(' ');
            let (nb, el) = (
                result.next().unwrap().trim().parse::<u64>().unwrap(),
                String::from(result.next().unwrap().trim()),
            );

            relation.insert(el, (nb, need));
        }

        Ok(Reaction {
            relation,
            remaining: HashMap::new(),
            used: HashMap::new(),
        })
    }
}
