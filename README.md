S3CS - **S3** **C**ache **S**hare
---
[![Build Status](https://travis-ci.com/yanganto/s3cs.svg?branch=master)](https://travis-ci.com/yanganto/s3cs)


#### :gear: :wrench: under developing :hammer: :gear:

There are a lot of application using S3 Bucket as cache, such as [sccache](https://github.com/mozilla/sccache)

This project builds a middle layer between S3 Bucket and application and actively push the cache object to other clients with same feature tags.

Besides, this project will act as a proxy, such that the [S3 version issue](https://github.com/mozilla/sccache/issues/281) of sccache can be solved.


