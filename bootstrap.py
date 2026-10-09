"""
Bootstrap script for Advent of Code project.

This script generates:
1. Year crates (crates/year_XXXX) with a Cargo.toml and a lib.rs that invokes the advent_year! macro
2. Individual day files with stub implementations:
   - Years 2015-2024: 25 days (day_one.rs through day_twenty_five.rs)
   - Years 2025+: 12 days (day_one.rs through day_twelve.rs)

The year crates are generated using the advent_year! macro, eliminating most boilerplate.
Only the individual day files with part_one/part_two function stubs still need generation.

A new year crate must also be added to the root Cargo.toml (`[dependencies]` and
`[workspace.dependencies]`), to `run_day` in src/main.rs, and to the `Year` enum in
crates/common/src/base.rs.

Note: Day 25 for years 2015-2024 only has part_one (no part_two).
"""

from __future__ import print_function

from pathlib import Path

NUMERALS_TO_STRING = {
    1: 'one',
    2: 'two',
    3: 'three',
    4: 'four',
    5: 'five',
    6: 'six',
    7: 'seven',
    8: 'eight',
    9: 'nine',
    10: 'ten',
    11: 'eleven',
    12: 'twelve',
    13: 'thirteen',
    14: 'fourteen',
    15: 'fifteen',
    16: 'sixteen',
    17: 'seventeen',
    18: 'eighteen',
    19: 'nineteen',
    20: 'twenty',
    21: 'twenty_one',
    22: 'twenty_two',
    23: 'twenty_three',
    24: 'twenty_four',
    25: 'twenty_five',
}

def get_days_for_year(year):
    """Return the number of days for a given year"""
    if year <= 2024:
        return 25
    else:
        return 12

def generate_year_manifest(year):
    """Generate the year crate's Cargo.toml, unless it already exists"""
    manifest_path = f'crates/year_{year}/Cargo.toml'

    if Path(manifest_path).exists():
        return

    print(f'Generating year manifest {manifest_path}')

    with open(manifest_path, 'w') as f:
        f.write('[package]\n')
        f.write('authors.workspace = true\n')
        f.write('edition.workspace = true\n')
        f.write(f'name = "year_{year}"\n')
        f.write('version.workspace = true\n')
        f.write('\n')
        f.write('[dependencies]\n')
        f.write('anyhow.workspace = true\n')
        f.write('common.workspace = true\n')

def generate_year_module(year):
    """Generate the year crate root (e.g. crates/year_2015/src/lib.rs) using the advent_year macro"""
    year_path = f'crates/year_{year}/src/lib.rs'

    # Always regenerate year modules to ensure they use the latest macro
    print(f'Generating year module {year_path}')

    num_days = get_days_for_year(year)

    with open(year_path, 'w') as f:
        # Generate macro invocation based on year-specific needs
        if year == 2019:
            f.write(f'common::advent_year!({year}, [computer]);\n')
        elif num_days == 12:
            f.write(f'common::advent_year_12!({year});\n')
        else:
            f.write(f'common::advent_year!({year});\n')

def main():
    # Add new years here as needed
    years = [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025]
    
    for year in years:
        src_path = f'crates/year_{year}/src'

        print(f'Creating directory {src_path} (if not exists)')
        Path(src_path).mkdir(parents=True, exist_ok=True)

        # Generate year crate
        generate_year_manifest(year)
        generate_year_module(year)

        num_days = get_days_for_year(year)
        for day in range(1, num_days + 1):
            day_string = NUMERALS_TO_STRING[day]
            day_path = f'{src_path}/day_{day_string}.rs'

            if not Path(day_path).exists():
                print(f'Creating file {day_path}')
                with open(day_path, 'w') as f:
                    f.write('use anyhow::anyhow;')
                    f.write('\n\n')
                    f.write('pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {')
                    f.write('\n')
                    f.write('\tErr::<usize, _>(anyhow!("Not implemented"))')
                    f.write('\n')
                    f.write('}')

                    # Day 25 for years 2015-2024 has no part_two
                    # For 2025+, all days have part_two
                    if not (year <= 2024 and day == 25):
                        f.write('\n\n')
                        f.write('pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {')
                        f.write('\n')
                        f.write('\tErr::<usize, _>(anyhow!("Not implemented"))')
                        f.write('\n')
                        f.write('}')

if __name__ == '__main__':
    main()
