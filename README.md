# Automated-Discord-Custom-Status
This is really similar to [my other project](https://github.com/justinas2314/AutomatedDiscordRichPresence) 
but the key difference is that instead of using the Rich Presence API this sets your custom status (and the way processes are found and matched is different).
# Disclaimers
I highly recommend looking through the code and making sure the script you give your discord token to (it's almost the same as giving away your password) does no harm.  
I don't recommend looking through the code too much as most of it is recycled from my older project which literally has a disclaimer `So don't expect the code to make sense`.  
This uses the windows API so it only works on windows.  
# Usage
First put discord account's token inside `token.txt`.
Compile `main.rs` and write custom commands inside `config.ini` (`config.ini` doesn't use .ini file syntax).  
There is an example `config.ini` file included.  
If you are going to run the .exe file move the config forder to the target (where the .exe file is) directory.  
If you are going to run via `cargo run --release` you don't need to move the file.  
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
# How to get your discord token?
There are plenty of tutorials online for example [this](https://github.com/Tyrrrz/DiscordChatExporter/wiki/Obtaining-Token-and-Channel-IDs).  
Here's a step by step guide (this is for Firefox but it's probably really similar on other browsers)  
1. Open [discord](https://discord.com/app) on your browser  
2. Press f12 to open developer tools and open `network` 
3. Find a request with an `Authorization` field in `Request Headers` (if you don't find anything refresh). The `Authorization` value is the token you need to put in `token.txt`.  
# Examples
### Example Problem
You want to display a custom status that is only set when watching youtube
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
