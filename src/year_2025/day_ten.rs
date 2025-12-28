use crate::year_2025::day_ten::model::MachineEntry;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let machine_entries = parse_machine_entries(input)?;

    let mut total_presses = 0;

    for machine_entry in machine_entries {
        let presses = search::search(&machine_entry)?;
        total_presses += presses;
    }

    Ok(total_presses)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let machine_entries = parse_machine_entries(input)?;

    let mut sum = 0;

    for machine_entry in machine_entries {
        sum += theorem::solve(&machine_entry)?;
    }

    Ok(sum)
}

fn parse_machine_entries(input: &str) -> anyhow::Result<Vec<MachineEntry>> {
    let mut machine_entries = Vec::new();
    let lines = input.lines().filter(|line| !line.is_empty());
    for line in lines {
        let machine_entry: MachineEntry = line.parse()?;
        machine_entries.push(machine_entry);
    }
    Ok(machine_entries)
}

mod model {
    use anyhow::{anyhow, bail};
    use itertools::Itertools;
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MachineEntry {
        lights: Lights,
        buttons: Buttons,
        joltage_requirements: JoltageRequirements,
    }

    impl MachineEntry {
        pub fn new(
            lights: Lights,
            buttons: Buttons,
            joltage_requirements: JoltageRequirements,
        ) -> Self {
            Self {
                lights,
                buttons,
                joltage_requirements,
            }
        }

        pub fn lights(&self) -> &Lights {
            &self.lights
        }

        pub fn buttons(&self) -> &Buttons {
            &self.buttons
        }

        pub fn joltage_requirements(&self) -> &JoltageRequirements {
            &self.joltage_requirements
        }
    }

    impl FromStr for MachineEntry {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let split = s.split(' ').filter(|s| !s.is_empty()).collect_vec();

            if split.len() < 3 {
                bail!("Expected at least 3 parts");
            }

            let lights = split[0];
            let lights = Lights::from_str(lights)?;

            let mut buttons = Vec::new();

            for i in 1..split.len() - 1 {
                let positions = Button::from_str(split[i])?;
                buttons.push(positions);
            }

            let buttons = Buttons::new(buttons)?;

            let joltage_requirements = split[split.len() - 1];
            let joltage_requirements = JoltageRequirements::from_str(joltage_requirements)?;

            Ok(MachineEntry {
                lights,
                buttons,
                joltage_requirements,
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Lights {
        lights: Vec<Light>,
    }

    impl Lights {
        pub fn new(lights: Vec<Light>) -> anyhow::Result<Self> {
            if lights.is_empty() {
                bail!("Must provide at least one light");
            }

            Ok(Self { lights })
        }

        pub fn len(&self) -> usize {
            self.lights.len()
        }

        pub fn toggle(&self, positions: &Button) -> anyhow::Result<Self> {
            let mut toggled = Vec::new();

            for (index, light) in self.lights.iter().enumerate() {
                if positions.contains(index) {
                    toggled.push(light.toggle());
                } else {
                    toggled.push(*light);
                }
            }

            Lights::new(toggled)
        }
    }

    impl FromStr for Lights {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.starts_with("[") {
                bail!("{s} must start with [");
            }

            if !s.ends_with(']') {
                bail!("{s} must end with ]");
            }

            let s = &s[1..s.len() - 1];

            let mut lights = Vec::new();

            for raw in s.split("") {
                if raw.is_empty() {
                    continue;
                }

                let light = Light::from_str(raw)?;
                lights.push(light);
            }

            if lights.is_empty() {
                bail!("must have at least one light");
            }

            Ok(Self { lights })
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Light {
        Off,
        On,
    }

    impl Light {
        pub fn toggle(&self) -> Self {
            match self {
                Light::Off => Light::On,
                Light::On => Light::Off,
            }
        }
    }

    impl FromStr for Light {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "." => Ok(Light::Off),
                "#" => Ok(Light::On),
                _ => Err(anyhow!("Unknown light: {}", s)),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Buttons {
        buttons: Vec<Button>,
    }

    impl Buttons {
        pub fn new(positions: Vec<Button>) -> anyhow::Result<Self> {
            if positions.is_empty() {
                bail!("Must provide at least one position");
            }

            Ok(Buttons { buttons: positions })
        }

        pub fn len(&self) -> usize {
            self.buttons.len()
        }

        pub fn iter(&self) -> impl Iterator<Item = &Button> + '_ {
            self.buttons.iter()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Button {
        positions: Vec<usize>,
    }

    impl Button {
        pub fn new(buttons: Vec<usize>) -> anyhow::Result<Self> {
            if buttons.is_empty() {
                bail!("Must provide at least one button");
            }

            Ok(Self { positions: buttons })
        }

        pub fn contains(&self, position: usize) -> bool {
            self.positions.contains(&position)
        }

        pub fn positions(&self) -> impl Iterator<Item = usize> + '_ {
            self.positions.iter().copied()
        }
    }

    impl FromStr for Button {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.starts_with("(") {
                bail!("{s} must begin with (");
            }

            if !s.ends_with(")") {
                bail!("{s} must end with (");
            }

            let s = &s[1..s.len() - 1];

            if s.is_empty() {
                bail!("must contain at least one button");
            }

            let mut buttons = Vec::new();

            for raw in s.split(',') {
                buttons.push(raw.parse()?);
            }

            Button::new(buttons)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct JoltageRequirements {
        requirements: Vec<usize>,
    }

    impl JoltageRequirements {
        pub fn new(requirements: Vec<usize>) -> anyhow::Result<Self> {
            if requirements.is_empty() {
                bail!("Must provide at least one joltage requirement");
            }

            Ok(Self { requirements })
        }

        pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
            self.requirements.iter().copied().enumerate()
        }
    }

    impl FromStr for JoltageRequirements {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.starts_with('{') {
                bail!("requirements must begin with {{");
            }

            if !s.ends_with('}') {
                bail!("requirements must end with }}");
            }

            let s = &s[1..s.len() - 1];

            if s.is_empty() {
                bail!("requirements must not be empty");
            }

            let mut requirements = Vec::new();

            for raw in s.split(',') {
                requirements.push(raw.parse()?);
            }

            JoltageRequirements::new(requirements)
        }
    }
}

mod search {
    use crate::year_2025::day_ten::model::{Button, Light, Lights, MachineEntry};
    use anyhow::bail;
    use std::collections::{HashSet, VecDeque};

    pub struct State {
        /// The current state of the lights
        lights: Lights,
        /// The previous positions that were pressed - empty for initial state
        previous_positions: Option<Button>,
        /// The running total of button presses
        total_presses: usize,
    }

    impl State {
        fn new(num_lights: usize) -> Self {
            let lights = Lights::new((0..num_lights).map(|_| Light::Off).collect()).unwrap();
            let previous_positions = None;
            let total_presses = 0;

            State {
                lights,
                previous_positions,
                total_presses,
            }
        }

        fn press_button(&self, button: Button) -> anyhow::Result<Self> {
            let lights = self.lights.toggle(&button)?;
            let previous_positions = Some(button);
            let total_presses = self.total_presses + 1;

            Ok(State {
                lights,
                previous_positions,
                total_presses,
            })
        }
    }

    pub fn search(machine_entry: &MachineEntry) -> anyhow::Result<usize> {
        let initial = State::new(machine_entry.lights().len());

        let mut queue = VecDeque::new();
        queue.push_back(initial);

        let mut seen = HashSet::new();

        while let Some(state) = queue.pop_front() {
            if &state.lights == machine_entry.lights() {
                return Ok(state.total_presses);
            }

            if !seen.insert(state.lights.clone()) {
                // This configuration of lights has been seen before - no use in continuing this branch
                continue;
            }

            for button in machine_entry.buttons().iter() {
                if let Some(previous) = &state.previous_positions
                    && previous == button
                {
                    // Pressing the same button twice takes us to a previous position with 2 more presses
                    continue;
                }

                let next = state.press_button(button.clone())?;
                queue.push_back(next);
            }
        }

        bail!("Desired Lights configuration not reached")
    }
}

mod theorem {
    use crate::year_2025::day_ten::model::MachineEntry;
    use anyhow::bail;
    use itertools::Itertools;
    use log::{debug, trace};
    use std::collections::HashMap;

    pub fn solve(machine_entry: &MachineEntry) -> anyhow::Result<usize> {
        let theorem = get_z3_theorem(&machine_entry)?;

        debug!(" === theorem ===\n{theorem}");

        let output = invoke_z3(&theorem)?;

        if !output.status.success() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                if !stdout.is_empty() {
                    trace!("z3 stdout:\n{stdout}\n");
                }
            } else {
                trace!("z3 stdout unavailable");
            }

            if let Ok(stderr) = String::from_utf8(output.stderr) {
                if !stderr.is_empty() {
                    trace!("z3 stderr:\n{stderr}\n");
                }
            } else {
                trace!("z3 stderr unavailable");
            }

            bail!("z3 did not exit successfully");
        }

        let stdout = std::str::from_utf8(&output.stdout)?;
        let stderr = std::str::from_utf8(&output.stderr)?;

        trace!("=== z3 stdout ===\n{stdout}\n");
        trace!("=== z3 stderr ===\n{stderr}\n");

        let value = stdout
            .lines()
            .find(|line| line.contains("(("))
            .and_then(|line| line.split_whitespace().last())
            .and_then(|s| s.trim_end_matches(')').parse::<usize>().ok())
            .ok_or_else(|| anyhow::anyhow!("Failed to parse z3 output"))?;

        Ok(value)
    }

    fn get_z3_theorem(machine_entry: &MachineEntry) -> anyhow::Result<String> {
        let mut theorem_lines = Vec::new();

        // Stores a mapping for which buttons will increment a given index
        let mut buttons_by_index = HashMap::new();

        for (index, button) in machine_entry.buttons().iter().enumerate() {
            theorem_lines.push(format!("(declare-const x{index} Int)"));
            theorem_lines.push(format!("(assert (>= x{index} 0))"));
            theorem_lines.push("".to_string());

            for position in button.positions() {
                buttons_by_index
                    .entry(position)
                    .or_insert_with(Vec::new)
                    .push(index);
            }
        }

        for (index, value) in machine_entry.joltage_requirements().iter() {
            let variables = buttons_by_index
                .get(&index)
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|idx| format!("x{idx}"))
                .join(" ");

            let assertion = format!("(assert (= (+ {variables}) {value}))");
            theorem_lines.push(assertion);
        }

        theorem_lines.push("".to_string());

        let minimize = machine_entry
            .buttons()
            .iter()
            .enumerate()
            .map(|(i, _)| format!("x{i}"))
            .join(" ");
        let minimize = format!("(minimize (+ {minimize}))");
        theorem_lines.push(minimize);

        theorem_lines.push("".to_string());

        theorem_lines.push("(check-sat)".to_string());
        theorem_lines.push("(get-objectives)".to_string());

        Ok(theorem_lines.join("\n"))
    }

    fn invoke_z3(theorem: &str) -> anyhow::Result<std::process::Output> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new("z3")
            .arg("-in")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(theorem.as_bytes())?;
        }

        child.wait_with_output().map_err(anyhow::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::model::*;
    use super::search;
    use std::str::FromStr;

    #[test]
    fn test_machine_entry_from_str() {
        assert_eq!(
            MachineEntry::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap(),
            MachineEntry::new(
                Lights::from_str("[.##.]").unwrap(),
                Buttons::new(vec![
                    Button::from_str("(3)").unwrap(),
                    Button::from_str("(1,3)").unwrap(),
                    Button::from_str("(2)").unwrap(),
                    Button::from_str("(2,3)").unwrap(),
                    Button::from_str("(0,2)").unwrap(),
                    Button::from_str("(0,1)").unwrap(),
                ])
                .unwrap(),
                JoltageRequirements::from_str("{3,5,4,7}").unwrap()
            )
        );
    }

    #[test]
    fn test_lights_from_str() {
        let s = "[.##.]";

        assert_eq!(
            Lights::from_str(s).unwrap(),
            Lights::new(vec![Light::Off, Light::On, Light::On, Light::Off]).unwrap()
        );
    }

    #[test]
    fn test_light_from_str() {
        assert_eq!(Light::from_str(".").unwrap(), Light::Off);
        assert_eq!(Light::from_str("#").unwrap(), Light::On);
        assert!(Light::from_str("0").is_err());
    }

    #[test]
    fn test_positions_from_str() {
        assert_eq!(
            Button::from_str("(2)").unwrap(),
            Button::new(vec![2]).unwrap()
        );
        assert_eq!(
            Button::from_str("(1,3)").unwrap(),
            Button::new(vec![1, 3]).unwrap()
        );
    }

    #[test]
    fn test_joltage_requirements_from_str() {
        assert_eq!(
            JoltageRequirements::from_str("{3,5,4,7}").unwrap(),
            JoltageRequirements::new(vec![3, 5, 4, 7]).unwrap()
        );

        assert_eq!(
            JoltageRequirements::from_str("{10,11,11,5,10,5}").unwrap(),
            JoltageRequirements::new(vec![10, 11, 11, 5, 10, 5]).unwrap()
        )
    }

    #[test]
    fn test_search() {
        let machine_entry =
            MachineEntry::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(search::search(&machine_entry).unwrap(), 2);

        let machine_entry =
            MachineEntry::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap();
        assert_eq!(search::search(&machine_entry).unwrap(), 3);

        let machine_entry = MachineEntry::from_str(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        )
        .unwrap();
        assert_eq!(search::search(&machine_entry).unwrap(), 2);
    }
}
