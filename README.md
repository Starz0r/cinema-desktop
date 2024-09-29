# Cinema Desktop App

## Roadmap

- [ ] Update Bootstrapping
- [ ] Other Platforms
	- [ ] Windows 10+
	- [ ] Apple macOS
	- [ ] GNU/Linux
	- [ ] Android
- [ ] Dark Mode
- [ ] Server List
- [ ] Use System Binaries (yt-dlp, ffmpeg, streamlink)
- [ ] External Media Player Support
- [ ] Livestream Playback via Streamlink

## Reasoning

### Why this and not a website?

I really tried hard to come up with methods to make it a website, but I found it most likely wouldn't be super-viable in the long term. As I didn't want people (mainly friends) having to install a application on their device to make this work, but I ran into too many problems that didn't have many good solutions. While this meant that I'd have broader platform support (Windows, macOS, Linux, and Android), it also meant I would be losing out on one platform (iOS/iPadOS), which I figured was worth the tradeoff.

1. Go through with the plan and use a proxy.

The biggest issue plaguing a in-browser web app is of course, CORS. CORS makes it impossible to query websites from my own without using a proxy, and, without a way to disable it, only left me with a few options. I thought about how I could make this work the most, but you'd need a lot of throttling and temporary storage. This also introduces a secondary server that would have to serve files, and it was all just really messy to do and make work, so I decided early on I wouldn't go this route.

2. Use a browser extension to handle the work.

While this would work in-theory, this still meant that 1. You still needed to install SOMETHING to get it to work, and 2. I'd still be beholden to some limitations. I'd still be on the web platform, and have to be careful about what limitations each browser imposes on extensions, which, while not hard to work around, found not worth it in the long run.

3. Create an installable application.

This is what I decided to go with. Not only do I get the most flexibility out of all the choices, but there is less limitations, and more room for innovations and features. Yes, this means I miss out on Apple-related mobile platforms, but I'd figured this was a good enough tradeoff for me, while still keeping me sane.

### What about the alternatives?

I like making my own things. On a more serious note, while I really like CyTube and W2G, they're very limited by the fact they can only play certain things. CyTube requires API keys, limiting both to a handful of curated services. By putting the software on the device, you can also make requests to that service as if you were visiting it yourself, no need to manage API keys. The amount of services that yt-dlp supports is immense, and only grows bigger.

Syncplay is the closest to what I actually wanted, but it isn't that beginner friendly. Firstly, it requires you to have a external media player, while my app doesn't. Second, it doesn't really support URLs all that well. It's possible, but getting it setup for all parties involved might require a guide, while you can just install my application and type out a URL and be done with it. Third, the codebase Not does not have a proper separation of concerns (the Qt desktop parts are merged with the server parts), a bit messy, and is poorly documented. I wanted to implement "vote-skipping" into the protocol, but could not really terse the sections of the code that would make this possible, especially without breaking things. A lot of the poor code hygiene isn't entirely their fault. It's a result of being a fairly old project, having ad-hoc modifications done to the project, and without enforcing proper standards on code changes means it's not in the best state. Though this isn't really an excuse, as they could always do some cleanup or rewriting, if they wanted to. However, I appreciate their efforts regardless!

Having my application be installable, also allows for more interesting and unique features, like being able to playback the video in a external media player, instead of the website. Or playing back from a torrent file! (Although you could do this on a website, I have yet to seen it done). This also allows me to cut down on some of the fat these services offer. We don't need text or voice chat built-in. Literally everyone and their mom probably uses Discord, Matrix, XMPP, IRC, or whatever weird esoteric chat service is offered these days. And we don't need to require users register (though, we do have other methods for abuse mitigation).

## Inspirations
	[Watch2Gether](https://w2g.tv)
	Synchtube.com (defunct March 2013)
	Rabb.it (defunct July 2019)
	[CyTube](https://github.com/calzoneman/sync/)
	[GMod Cinema](https://steamcommunity.com/sharedfiles/filedetails/?id=2419005587)
	[Syncplay](https://syncplay.pl/)
	[Swamp Cinema](https://swamp.sv/) *CW: Politics, Edgy Humor*