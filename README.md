aw-leaderboard
==============

[![Build](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml/badge.svg)](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml)

A public leaderboard for ActivityWatch data.

Inspired by the [WakaTime leaderboard](https://wakatime.com/leaders).

Built with FastAPI, SQLalchemy, PostgreSQL. (**note:** this stack might change)


## Broad overview

A user runs ActivityWatch on their computer, but wants to share how much they've worked on X.

They go into the "share" section of the web UI, authenticate with the leaderboard service, and then get to choose what to share.

What to share is configured by specifying things like relevant categories and their patterns, flooding parameters, etc.

Either we let the user specify everything themselves, or we give them a list of options from already existing leaderboard categories. (not sure what the smartest thing is here)

When the user is happy with their choices, the data is pulled and shown for review, then finally events are sent with all event data stripped except the category names (the `$category` key in event data).


## Goals

 - Serve as a website for people to share how much time they've spent on different things (projects, communities, games).
   - One example in particular: allow contributors to share how much time they've spent working on/in ActivityWatch.
   - Users could want this for several reasons, like: bragging rights, accountability (social media overuse).
 - Modified version should be able to function as a company/team server that members can report time to.
 - Modified version should be able to function as a research server that study participants can report time to.

Secondary goals:

 - Make ActivityWatch more social by letting users share their stats
   - Make it something people can "show off" online or in their profiles (with badge/link), helps improve reach.
 - A way to track contributor activity and pay out salary based on time spent


## Considerations

 - How will/should multiple devices be handled?


## Related issues

 - Implement reporting: https://github.com/ActivityWatch/activitywatch/issues/233
 - Telemetry: https://github.com/ActivityWatch/activitywatch/issues/120
   - Could help us establish yet another "active user" number (in addition to browser extension use stats)
