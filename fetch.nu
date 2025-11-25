#!/usr/bin/env nu

def main [y: int, d: string] {
    let year = $y;
    let day = ($d | str replace  --all '^0+' '' | into int);
    let url = $"https://adventofcode.com/($year)/day/($day)/input";
    let cookie = ($env.AOC? | default "");
    
    if $cookie == "" {
        echo "Please set the AOC environment variable to your session cookie.";
        echo "It should look like: session=abacbacbacb";
        exit 1;
    }
    
    let input = http get --headers [Cookie $cookie] $url;
    let year_short = ($year mod 100 | into string);
    let padded_day = ("00" + ($day | into string) | str substring (-2..));
    let benches_path = $"y($year_short)/d($padded_day)/input.txt";
    
    (echo $input) out> $benches_path;
}