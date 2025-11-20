+++
title = "The AeroRust community - 3 years birthday 🎉"
description = """
Welcome to the first AeroRust community blog post,
I want to take this special occasion and talk about the community work in the past year and reflect on the last 3 years.

It's time to reveal the 2023 Roadmap, so happy anniversary to all of you - new and current members and contributors, and happy reading...
"""

[extra]
draft = false
show_toc = true
# youtube thumbnail of "Exploring Rust in Robotics"
meta_image = "https://i3.ytimg.com/vi/yc1OWMkde8w/sddefault.jpg"

[[extra.author]]
name = "Lachezar Lechev"
twitter = "elpiel_"
github = "elpiel"
+++

Today we mark the 3 year anniversary since we've created this community and we wouldn't be here if it wasn't for the many people along the way that join and contribute to our efforts. Thank you ♥ for your continues support!

<!-- more -->

## How it all began

Exactly 3 years ago, on 2.1.2020, I was having a chat with Dylan ([@Dylan-DPC](https://github.com/Dylan-DPC)), in the span of a few days prior to this, about creating a Rust community specifically focused on the aerospace application of the language.
A gentle push from him led to the creation of the Discord server ([which you can join](https://discord.gg/Xd98GjuEhu)) and announcing the community on [reddit](https://www.reddit.com/r/rust/comments/ejdv7w/announcing_aerorust_the_unofficial_working_group/) a day later.

## What happened in 2022

##### Meetups

Earlier in the year (in March, to be precise) we had the [Rust London meetup takeover](https://skillsmatter.com/meetups/13826-ldn-talks-march-2022-aerorust-takeover) with some great talks about robotics, video stabilization and qualification for safety-critical systems. All talks are available on our Youtube channel:

{{ youtube(id="yc1OWMkde8w", playlist="PL6S-AaFWAAfmyj_n7XmleBx9_hlX2Nx0E", class="youtube my-4" ) }}

##### Hacktoberfest

[Hacktoberfest](https://hacktoberfest.com/) which takes place in November, in my opinion, was a success!

We saw a few contributors to the [`nmea` crate](https://github.com/AeroRust/nmea) (a NMEA 0183 maritime data protocol parser) after promoting [issue #54](https://github.com/AeroRust/nmea/issues/54) - a `good first issue` to add support for more sentences.

But... I also want to be straight with you - it's far from what I had envision where we will be at after 3 years.
It has been challenging to find contributors as it has been exacerbated by the COVID-19 pandemic in 2020 and the lack of an onboarding process for community members.

## Rust Foundation's Project Grants (update)

In 2022, we've applied to the second round of the Project grants (in November) with the goal to sponsor 2 people which will work on the Roadmap for 2023 in the first quarter of the year. They would have been responsible mainly for the organisational part of the community, preparing 2 events and creating the onboarding process, which I've mentioned above. They were also going to be responsible for finding new sponsorships allowing them to continue their work after the proposed timeframe of 3 months.

Our proposal, sadly, was not accepted this time, however, we've decided to start working on some of the points and look for contributors.

The feedback that we received was that this round they received a ton of applications and that made things very competitive. This is great news for the Rust project in general and I fully support their decisions hoping that things will grow even more from here on.

The main feedback that we received, however, was that they were looking for "direct impact on the Rust project itself, or broad impact across the ecosystem" (because of the many applications) and "aerospace just didn't fall into that category".

I want to say directly that I disagree but I do understand them. There are many reasons why I think that the aerospace domain is what can get Rust on a whole new level but the main takeaway from this feedback to me is that we have to change this view in the Rust community and beyond. As a multidisciplinary domain, aerospace is very challenging and capital intensive which makes Rust adoption a no-brainer (in the long run) and will bring tangible benefits for the whole ecosystem and the Rust project.

Since we strive to be as open as possible, you can read the full proposal in [this document](https://docs.google.com/document/d/10EX_yt1MTL1DmzGXMm3hn2LIWu4xOsZRRI7_QGMx3oc).

I also want to take this time to say a big **THANK YOU ♥** to [Alice Cecile](https://github.com/alice-i-cecile) for providing an in-depth feedback, and Archie Tilbrook, Spencer Imbleau and a few other people for reading through the proposal.

Also, thank you ♥ [Sage Griffin](https://github.com/sgrif) for providing feedback on why our proposal was not accepted in this instalment of the grants.

## Roadmap 2023

We're looking to grow the efforts in the aerospace industry from our community and the Rust project as a whole by outlining the benefits that the Rust language, ecosystem and communities can bring to the table.

In this line of thought, we will be focusing on the following 3 goals:

- Outreach - Reaching out to projects, maintainers, organisations, companies, boosting social media, finding content creators and organising events.
- Onboarding - Create an onboarding process with information on how people can contribute, list of `good first issue`s and so on.
- Project Management - Projects planning and management (issues, PRs, mentoring, answering questions, coordination, etc.) for current and new projects.

### Outreach

The Outreach goal will take up most of efforts in this year as it will be the one that incorporates the organisational and execution parts for the rest of the goals.

At the end of 2023 we're aiming to have:

- Reached out to organisations for collaboration on current or future projects, events, promoting efforts, etc.
- 2 events - a Workshop and a Hackathon/Game jam
- 1 technical blog post per month with eventual re-posting to [RustJobs](https://rustjobs.dev) or [embeddedcomputing.com](https://embeddedcomputing.com).
- More regular social media updates and blog posts for the ongoing efforts

#### Events

We think that creating these 2 events will help with our outreach goal for this year and excite the people about the ongoing efforts in the community.

Do not hesitate to contact us if you or your company are interested in sponsoring one of our events or the community efforts in general.

##### Embedded CubeSat workshop

In December 2022, we've started preparations for an embedded workshop that will mimic a satellite subsystem, hopefully taking place in mid-2023.

The **community** goal for this workshop is to spread the word of the AeroRust efforts and draw more people and organisations in the long run and, of course, promoting Rust as a tool for developing aerospace applications.

The **main** goal is to give participants an opportunity to get to know embedded programming while giving a _practical_ example and exercise to them. It's good to mention, that our goals do **not** include creating a real and production ready subsystem.

Another very important goal that we've set up for the workshop is to make it as **affordable** and **accessible** as possible.

**Location**

Since most of my connection are here in Bulgaria (I live in Plovdiv, Bulgaria, for those unaware), we hope to make the first workshop either in Plovdiv or Sofia.

However, we're still looking for **other locations** where we can make this workshop a reality after the initial one in Bulgaria.
If some of you are interested to collaborate on this workshop and make an event for your city or country, do reach out to express your interest. We already saw interest from people in Kenya and it would be amazing to execute this event at multiple locations, making it global, like our community!

We still haven't decided what the end system will look like or the hardware that we will use so **stay tuned for updates** in the very near future including the **location** and **time** when the **first** workshop will take place in Bulgaria!

##### Hackathon / Game Jam

We saw the interest by ESA to use game engine techniques for modernising On Board software (see [In other news...](#in-other-news)) with Rust!

Game development can be as fun as playing games but it can also be a place that merges the development process with the educational aspect of the game itself (like, for example, flight or space simulators).

I personally think that game development is a great tool for learning but also I believe that it is a good funnel for people that want to get into programming.
It brings the pleasure of creating games (or game-like applications) while, at the same time, you learn and can be directly applied to real life experiences (again, simulators).

This event hasn't been on focus for us (yet) as it's slotted for the last quarter of 2023 so many details are still to be decided and it really depends on the contributors we manage to find for it.

All I can say for now is that the the Bevy community is open to promoting such an event and we need to find a good theme for it. We've already been discussing a few ideas but nothing is set, as I already mentioned.

### Onboarding process

This has been the most challenging part since the creation of the community - the **onboarding process**.

A place where people can familiarize themselves with the different disciplines that make up the aerospace industry, like Guidance Navigation and Control, RTOS, space and aerospace protocols and standards, existing in the Rust ecosystem projects and many, many more.
On the other hand, it should be tightly coupled with some of the efforts happening in the community - the ["Are we in space yet?" crates catalogue](@/catalogue/_index.md), the [`nmea`](https://github.com/AeroRust/nmea) and [`mav-sdk`](https://github.com/AeroRust/mav) crates, in order to bring more collaborators and contributors to the community.

Apart from the general overview of the aerospace industry, it should include a good starting point for beginners or experienced people to engage with the community and contribute back, like a list of `good first issue`s, calls for participation, calls for content writing and even community organisational tasks.

### Project Management

The aerospace domain is huge and, as I said before, it's multidisciplinary, incorporating physics, mathematics, fluid dynamics, even AI and many more disciplines. As such we haven't focused on anything specific in the community until now and I personally want to change that.

While talking about the onboarding process, you might have realized or already know that there's a lot of organisational work that goes into maintaining a project, let alone a whole community. Project management is a wide term but we want to focus our attention to **coherent results**.

This means that we should be building to something greater instead of focusing on scattered crates and project in various, somewhat related, areas. We need to be focusing on issues in such a manner that we're building towards this greater goal and coordinate our efforts across projects, issues and PRs.

One big part of this can be mentoring contributors, slowly building towards this goal and I hope that the embedded workshop is just the start to get to these results so we need to carve out a solid foundation!

There have been many good ideas shared in the community throughout these 3 years but without people invested in making them a reality, they are just that - ideas.

We need to focus on the clear benefits and have people overlooking the processes of the community and the projects.

What should be building towards?

Well.. after years, I still don't know what that looks like but I do know one thing: We should aim higher and be bold.

The Rust adoption in aerospace can get an enormous boost from our efforts and bring even more resources in the Rust project than ever before.

There have been other communities, like the [Libre Space Foundation](https://libre.space) which slowly, over time, built many projects from their Ground Stations network ([SatNOGs](https://satnogs.org)) to their own CubeSat missions (like [QUBIK](https://libre.space/2022/11/01/librespace-foundation-back-in-space)).

So.. Do you have an idea you're ready to follow through?

I wanted to also share [this excellent talk](https://www.youtube.com/watch?v=u3PJaiSpbmc) by Alice at RustConf 2022 about why Open source projects need a project management.

## We need you!

As you've seen so far, we have a very bold roadmap for 2023, which seems a bit vague at this point, and this is why we need you!

It will be close to impossible to make it a reality if we are only a handful of people juggling between all the tasks, whenever we find time for them.

And it **does not** matter if you have a technical background or not, aerospace experience or even Rust experience for that matter. We want to grow this community and establish a healthy environment for Open Source which requires a lot of efforts to make it sustainable.

If you're interested to join the efforts and guide the community to a successful 2023, reach out to us and join the discussions on Discord or via email at [aerospace.rust@gmail.com](mailto:aerospace.rust@gmail.com).

## In other news...

ESA has accepted a few project ideas proposals to evaluate Rust for space on-board applications:

- [Using game engine techniques and Rust to modernize On Board software](https://ideas.esa.int/servlet/hype/IMT?documentTableId=45087137960624414&userAction=Browse&templateName=&documentId=53298a0c4c4b392accb7c6ea5ba971db)
- [cRustacea in Space - Co-operative Rust and C embedded applications in Space - Theory and Practice](https://ideas.esa.int/servlet/hype/IMT?documentTableId=45087137960624414&userAction=Browse&templateName=&documentId=6acab7a7ba0da1ea05c864552665bcb5)
- Evaluation of Rust usage in space application by developing BSP and RTOS targeting SAMV71 (link will be added when ESA publishes the project idea)
