FROM ubuntu:23.10

RUN dpkg --add-architecture i386
RUN apt-get update
RUN apt-get install -y --no-install-recommends libc6-dev gcc
RUN apt-get install -y --no-install-recommends \
  file \
  wget \
  ca-certificates \
  python3 \
  python3-distutils \
  unzip \
  expect \
  openjdk-8-jre \
  libstdc++6:i386 \
  libpulse0

WORKDIR /android/
COPY android* /android/

ENV ANDROID_ARCH=aarch64
ENV PATH=$PATH:/android/ndk-$ANDROID_ARCH/bin:/android/sdk/cmdline-tools/tools:/android/sdk/platform-tools

RUN sh /android/android-install-ndk.sh $ANDROID_ARCH
RUN sh /android/android-install-sdk.sh $ANDROID_ARCH
RUN mv /root/.android /tmp
RUN chmod 777 -R /tmp/.android
RUN chmod 755 /android/sdk/cmdline-tools/tools/* /android/sdk/emulator/qemu/linux-x86_64/*

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android28-clang \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_RUNNER=/tmp/runtest \
    CC_aarch64_linux_android=aarch64-linux-android28-clang \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
ENTRYPOINT [ \
  "bash", \
  "-c", \
  # set SHELL so android can detect a 64bits system, see
  # http://stackoverflow.com/a/41789144
  "SHELL=/bin/dash /android/sdk/emulator/emulator @aarch64 -no-window & \
   rustc /tmp/runtest.rs -o /tmp/runtest && \
   exec \"$@\"", \
  "--" \
]
