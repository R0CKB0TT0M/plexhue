# plexhue
A Rust Application to turn off hue lights when you start a Movie in Plex and turn them back on after.


Installation:
1. download the latest Version from releases.
2. on your plex server go to settings/webhooks and insert 127.0.0.1:8000 if plexhue will run on the same computer. else insert \<plexhuePcIp\>:8000
3. open the folder with plexhue.exe in terminal.
4. run <code>.\\plexhue.exe getuuid</code> then start a video on the player you want to set up
5. plexhue should now output the id of that player copy that for later then press ctrl + C  to end execution
6. next run <code>.\\plexhue.exe setup</code> and follow the instructions.
7. you should now have a config.json in the same folder.

Congratulations you are now done with the setup.
you can now run <code>.\\plexhue.exe server</code> and leave this running and your hue lights should turn on or off with play/pause events from your configured player.


> if you want an icon to start plexhue on your desktop instead of using the command line do the following:
> 1. after setup right click on plexhue.exe and click create shortcut.
> 2. now right click on the shortcut and go to properties
> 3. in the target field add a space followed by server to the end.
> 
> your shortcut will now start you server sit back and enjoy your movies.
