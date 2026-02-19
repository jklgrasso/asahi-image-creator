
# asahi-image-creator

##### (I guess it's actually an *iso* creator but too late)

---

Currently creates a loop device from a given file. Doesn't do much else.

Will:

- mount RO iso as loop device
- create images of specific fs dirs (efi, boot, root, etc)
- make iso from said images

---

I have no idea what a Mac system needs/expects to install a functional OS. I have no idea how the partitions need to be setup. Even if I did, how do I tell the OS what to expect, and how to handle installation?

Inspiration from [ubuntu-asahi](https://ubuntuasahi.org/). I don't want to look at how it works too much. I like figuring things out. I'm very new to Rust still, so sorry for the gross code.


Also, when this is running, it uses one thread @ 100%. Not sure why yet.

