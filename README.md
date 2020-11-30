# Automated-Discord-Custom-Status
This is really similar to [my other project](https://github.com/justinas2314/AutomatedDiscordRichPresence) 
but the key difference is that instead of using the Rich Presence API this sets your custom status.
# Usage
Compile main.rs and write custom commands inside `config.ini` (`config.ini` doesn't use .ini file syntax).  
There is an example config.ini file included.  
If you are going to run the .exe file move the config forder to the target (where the .exe file is) directory.  
If you are going to run via cargo run --release you don't need to move the file.  
The following arguments can be used:
* `text` static string which will be used as a custom status
* `emoji` static emoji which will be used as the custom status emoji
* `regex` used with `format`  
the regex to match the window's text and split it into groups. Example: `(.) and (.)` would match and group `one and two` as 
```
{
    0: "one and two",
    1: "one",
    2: "two"
}
```

* `format` used with `regex`  
this is a string used to put the groups into one piece. Example: `{2} and {1}`. The number inside `{}` is the group's index (indexing starts from 1, 0 is the entire matched regex). If group 2 matched to `two` and group 1 matched to `one` the final string will be `two and one`.
* `fallback` optionally used with `format` and `regex` if nothing is matched  
if `regex` and `format` don't match anything this will be the status instead
* `fallback_emoji` used together with `fallback` as the emoji  
# Examples
### Example Problem
You want to display a custom status when watching youtube that is only set when watching youtube
### Solution
1. Find a regex that matches the browser window with youtube and write it down  
`".* YouTube .*"`  
2. Set `emoji` and `text` to whatever you want (I'll leave `emoji` blank here)  
`text = watching youtube`  
The entire script will look like  
```
".* YouTube .*"
text = watching youtube
```
### Example Problem
You want your custom status to have the name and artist of a song you're listening to on spotify
### Solution
1. Find a regex that matches the spotify window and doesn't match other windows   
`"^[^-]*? - (?!Mozilla)(?!IntelliJ)(?!Discord)[^-]*$"`  
This regex exludes windows like IntelliJ IDEA, Discord and Mozilla Firefox but matches everything else in a format `text - more text`
2. Set `regex` to group the important parts of the window's name  
`regex = (.*) - (.*)`
3. Set `format` to put together the final text  
`format = listening to {2} by {1}`
4. Set `emoji`  
`emoji = 🎵`  
The entire script will look like
```
"^[^-]*? - (?!Mozilla)(?!IntelliJ)(?!Discord)[^-]*$"
regex = (.*) - (.*)
format = listening to {2} by {1}
emoji = 🎵
```
