aw-leaderboard
==============

[![Build](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml/badge.svg)](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml)

A public leaderboard for ActivityWatch data.

Inspired by the [WakaTime leaderboard](https://wakatime.com/leaders).

Built with Rust using Rocket and (very basic) tera2 templates (for now).


## Overview

The `aw-leaderboard` application allows ActivityWatch users to share activity data through a "share" section in the web UI. After authenticating with the leaderboard service, users can choose the type of data to share by configuring relevant categories, patterns, and other parameters.

Users can either define these configurations themselves or select from a list of options predefined by existing leaderboard categories.

Once users are satisfied with their selection, the data is pulled for review. Only the event data stripped to the category names (the `$category` key in event data) is sent and displayed.


## Goals

The main goal is to:

 - Provide a platform for users to share their activity times across different categories. Like projects, communities, or games.
   - For example, contributors can use it to report the time spent working on ActivityWatch. (dogfooding)

Secondary goals:

 - Enhance the social aspect of ActivityWatch by enabling users to share their stats.
   - Promote user engagement by providing badges/links for online sharing or profile display, which may also increase our reach.
 - Make ActivityWatch more social by letting users share their stats
   - Make it something people can "show off" online or in their profiles (with badge/link), helps improve reach.
 - Develop a system to track contributor activity and disburse payments based on time spent.
 - Serve as a foundation that could be forked and modified to function as a reporting server for teams, companies, and research studies.


## Why do users want to share data?

Users may wish to share their data for a variety of reasons, such as:

- To maintain accountability, like ensuring they complete their thesis or prevent social media overuse.
- To showcase their productivity.
- To demonstrate their contributions to a specific project or community.


## Considerations

 - How will/should multiple devices be handled?
 - Better name
   - We need a better name than "aw-leaderboard".


## Related issues

 - Implement reporting: https://github.com/ActivityWatch/activitywatch/issues/233
 - Telemetry: https://github.com/ActivityWatch/activitywatch/issues/120
   - Could help us establish yet another "active user" number (in addition to browser extension use stats)
