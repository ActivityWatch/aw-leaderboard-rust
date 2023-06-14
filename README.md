aw-leaderboard
==============

[![Build](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml/badge.svg)](https://github.com/ActivityWatch/aw-leaderboard/actions/workflows/build.yml)

A public leaderboard for ActivityWatch data.

Inspired by the [WakaTime leaderboard](https://wakatime.com/leaders).

Built with Rust using Rocket and (very basic) Tera templates (for now).


## Overview

The `aw-leaderboard` application allows ActivityWatch users to share their time-tracking data on an opt-in basis.

Users share their data by registering an account on the site, obtaining a API key, and setting up data uploading through a "share" section in the ActivityWatch web UI.

Users can configure what data to share by configuring relevant categories, patterns, and other parameters.
Users can either define these configurations themselves, or select from a list of defaults.

Once users are satisfied with their selection, the data is pulled for review. Only the event data stripped to the category names (the `$category` key in event data) is sent and displayed.


## Goals

The main goal is to:

 - Provide a platform for users to share their device activity, across different categories.
   - Example: people can share how much they work, or spent on social media. (to flex/stay accountable)
   - Example: contributors can use it to report the time spent working on different projects, like ActivityWatch. (dogfooding)
   - Example: parents can set up reporting of time spent on video games, etc. (questionable usecase, no point in uploading really)
   - Example: researchers can ask their subjects to set up reporting for categories of interest.

Secondary goals:

 - Make ActivityWatch social by letting users share their stats
   - Make it something people can "show off" online or in their profiles (with badge/link), helps improve reach.
   - Promote user engagement by providing badges/links for online sharing or profile display, which may also increase our reach.
 - Develop a system to track contributor activity and disburse payments based on time spent.
 - Serve as a foundation that could be forked and modified to function as a reporting server for teams, companies, and research studies.


## Why do users want to share data?

Users may wish to share their data for a variety of reasons, such as:

- To maintain accountability, like ensuring they complete their thesis or prevent social media overuse.
- To showcase their productivity.
- To demonstrate their contributions to a specific project or community.


## Considerations

 - How will/should multiple devices be handled?
   - We will simply implement uploads/reporting on a device-by-device basis, where each device has to register and report. 
     - Gives us the ability to query "last updated" for each device (very helpful) 
   - Data from each device is then combined.
 - Better name
   - We need a better name than "Leaderboard", maybe?
     - Esp. considering most uses are not really competitive.


## Related issues

 - Implement reporting: https://github.com/ActivityWatch/activitywatch/issues/233
 - Telemetry: https://github.com/ActivityWatch/activitywatch/issues/120
   - Could help us establish yet another "active user" number (in addition to browser extension use stats)
