+++
# Make sure to remove all keys that you don't need as well as all the comments
title = "Your blog post title"
# You can also override the default blog post template, but that's not required
# template = "blog_post.html"
# It is used as the description of the Blog post in the list
description = "Description of your blog post for the blog listing"
# The date is set by the filename but we need to use it here,
# as the file does not contain a valid date "YYYY-MM-DD"
# This is not required if the filename contains a valid format!
date = "2022-01-01"
# Is it still a draft?
draft = true

[extra]

# Show a "Table of contents" before the content of the blog post
show_toc = true

[[extra.author]]
name = "The AeroRust community"
# The author's twitter account
twitter = "AeroRust"
# The author's Github account
github = "AeroRust"
+++

Text of your blog goes here...

If you want to add a "Continue Reading" link with a partial quote from the blog post in the list of blog posts, add this line where you want the quote to end:
<!-- more -->

And continue with your blog post content here.