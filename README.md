# AutomatedDiscordRichPresence
Create custom Discord Rich Presences that toggle on while a certain app is running.  
# Disclaimers
This is not a 'serious' project. I am doing this because I've started learning Rust and wanted to write something in it. So don't expect the code to make sense.  
Only works on Windows.  
Non ASCII Strings are buggy because the code needs to communicate with windows using commands and windows does not (atleast for me) output the chars correctly.  
# Usage
Compile main.rs and write custom commands inside `config.ini`.  
There is an example `config.ini` file included.  
If you are going to run the .exe file move the config forder to the target (where the .exe file is) directory.  
If you are going to run via `cargo run --release` you don't need to move the file.  
The following arguments can be used:  
 details  
 state  
 large_image  
 small_image  
 large_text   
 small_text  
The first line is where the preset will be defined. It can be written like this `[firefox.exe, -private]`.
Here the code will look for a window that includes 'firefox.exe' but does NOT include 'private'.  
This is not case sensitive. 
If you want to make sure the string '-private' is included in the app window write `[firefox.exe, \-private]`.  
To leave the place as a placeholder write `something = ..`.  
If details is left as `..` the full name of the window will be used.  
If state is left as `..` it will be used to fit a part of the description if the description is longer than 20 chars.  
If the images are left as placeholders they will not be used.  
Image text left as placeholder will also change to the full name of the window.  
We can also inherit placeholders.  
If we define a template as `![TEMPLATE]`  
`state = this is a state`  
and then pass it to a different preset `[preset] <- [TEMPLATE]` we will not have to define the state again.  
This is useful for setting the same value in multiple presets without writing too much.  
We can also define the template as `[TEMPLATE]` but the template would then match all windows that include 'template'.  
'!' excludes the template from getting matched to window titles.
# Notes
Discord lets you use up to 150 images currently only 53 are being used  
I will probably add more images in the future  
# Images that can be used right now
androidstudiologo  
arduinologo  
blenderlogo  
chromelogo  
clogo  
cpplogo  
crunchyrolllogo  
csharplogo  
curiouscatlogo  
discordlogo  
eclipselogo  
excellogo  
facebooklogo  
firefoxlogo  
gimplogo  
githublogo  
gmaillogo  
hululogo  
instagramlogo  
intellijidealogo  
ituneslogo  
javalogo  
javascriptlogo  
libreofficelogo  
linkedinlogo  
mangadexlogo  
messengerlogo  
netflixlogo  
photoshoplogo  
powerpointlogo  
premiereprologo  
pycharmlogo  
pythonlogo  
readthedocslogo  
redditlogo  
rustlogo  
skypelogo  
slacklogo  
snapchatlogo  
spotifylogo  
stackoverflowlogo  
teamspeaklogo  
telegramlogo  
twitchlogo  
twitterlogo  
visualstudiocodelogo  
visualstudiologo  
vlclogo  
whatsapplogo  
wikipedialogo  
wordlogo  
youtubelogo  
zoomlogo  
