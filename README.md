# Library that retrieves city-time from around the world

## Features: 
1. Works with the following cities: Moscow, Budapest, Vienna, Berlin, London, Sydney, Brisbane, Melbourne, Hobart, Adelaide, Darwin, Perth, Los Angeles, Chicago, New York, Auckland
1. Library functionality: 
    - Enum CityList lists all supported cities as variants; 
    - A new city: City::new(CityList::variant) -> City, where CityList::variant is one of the CityList enum variants, i.e. a city; 
    - To retrieve a city from City struct: pub fn city(&City) -> &CityList; 
    - To retrieve a year from City struct: pub fn year(&City) -> i32; 
    - To retrieve a month from City struct: pub fn month(&City) -> u32; 
    - To retrieve a day from City struct: pub fn day(&City) -> u32; 
    - To retrieve hours from City struct: pub fn hours(&City) -> u32; 
    - To retrieve minutes from City struct: pub fn minutes(&City) -> u32; 
    - To retrieve seconds from City struct: pub fn seconds(&City) -> u32; 
    - Current date and time in the City: pub fn current_time(&City) -> String; 
    - Current UNIX timestamp: pub fn timestamp(&City) -> i64; 
    - Time difference between two cities: pub fn time_difference(&City_1, &City_2) -> f32;

## How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    weltzeit = {git = "https://github.com/azavgo/weltzeit", branch = "main"}
```
2. This example demonstrates how to find a time difference between Sydney and Auckland:   
```Rust
use weltzeit::*;
fn main() {
    let sydney = City::new(CityList::Sydney);   
    let auckland = City::new(CityList::Auckland); 
    
    dbg!(sydney.time_difference(&auckland));    
}
```