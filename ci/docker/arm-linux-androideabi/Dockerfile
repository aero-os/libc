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

ENV ANDROID_ARCH=arm
ENV PATH=$PATH:/android/ndk-$ANDROID_ARCH/bin:/android/sdk/cmdline-tools/tools:/android/sdk/platform-tools

RUN sh /android/android-install-ndk.sh $ANDROID_ARCH
RUN sh /android/android-install-sdk.sh $ANDROID_ARCH
RUN mv /root/.android /tmp
RUN chmod 777 -R /tmp/.android
RUN chmod 755 /android/sdk/cmdline-tools/tools/* /android/sdk/emulator/qemu/linux-x86_64/*

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER=arm-linux-androideabi-gcc \
    CARGO_TARGET_ARM_LINUX_ANDROIDEABI_RUNNER=/tmp/runtest \
    CC_arm_linux_androideabi=arm-linux-androideabi-gcc \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
ENTRYPOINT [ \
  "bash", \
  "-c", \
  # set SHELL so android can detect a 64bits system, see
  # http://stackoverflow.com/a/41789144
  "SHELL=/bin/dash /android/sdk/emulator/emulator @arm -no-window & \
   rustc /tmp/runtest.rs -o /tmp/runtest && \
   exec \"$@\"", \
  "--" \
]
