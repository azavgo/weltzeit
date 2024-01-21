# Library that retrieves city-time from around the world

## Features: 
1. Works with the following cities: Moscow, Budapest, Vienna, Berlin, London, Sydney, Brisbane, Melbourne, Hobart, Adelaide, Darwin, Perth, Los Angeles, Chicago, New York, Auckland
1. Library functionality: 
    - Enum CityList lists all supported cities as variants; 
    - A new city: City::new(CityList) -> City; 
    - To retrieve city: pub fn city(&City) -> &CityList; 
    - To retrieve year: pub fn year(&City) -> i32; 
    - To retrieve month: pub fn month(&City) -> u32; 
    - To retrieve day: pub fn day(&City) -> u32; 
    - To retrieve hours: pub fn hours(&City) -> u32; 
    - To retrieve minutes: pub fn minutes(&City) -> u32; 
    - To retrieve seconds: pub fn seconds(&City) -> u32; 
    - Current date and time: pub fn current_time(&City) -> String; 
    - Current UNIX timestamp: pub fn timestamp(&City) -> i64; 
    - Time difference between two cities: pub fn time_difference(&City, &City) -> f32;

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