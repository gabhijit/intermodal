//! Test Data used for Testing - Reference Manifests, Blobs etc.
//!
//! This test data is to be used with mocking.

pub(super) const DOCKER_LIST_MANIFEST_BLOB: &str = r#"{"manifests":[{"digest":"sha256:fdf235fa167d2aa5d820fba274ec1d2edeb0534bd32d28d602a19b31bad79b80","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"amd64","os":"linux"},"size":529},{"digest":"sha256:12cea180e80e7f5b8847a82d35b4bd6a8089703f4c17d662512e49884146fa45","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"arm","os":"linux","variant":"v7"},"size":529},{"digest":"sha256:6678ea27e7c7e3fce4adbc08d84cfef3d04211ee8e14e128acf6571b331a068a","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"arm64","os":"linux","variant":"v8"},"size":529},{"digest":"sha256:2237bb2b8b11cb0858e9c3c791ce1cf45035122ad7f065fb2d723ca2ad0a822d","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"ppc64le","os":"linux"},"size":529},{"digest":"sha256:153fa742cea71512a14224856538bcba53bb201f630304fd82abd78806fd1cf1","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"s390x","os":"linux"},"size":529}],"mediaType":"application\/vnd.docker.distribution.manifest.list.v2+json","schemaVersion":2}"#;

pub(super) const DOCKER_IMAGE_CONFIG_BLOB: &str = r##"{"architecture":"amd64","config":{"Hostname":"","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","DISTTAG=f33container","FGC=f33","FBR=f33"],"Cmd":["/bin/bash"],"Image":"sha256:2ad0e7e335b63ed128ee487e6140eb731c6202b9b1adf02da3c1a5a605028dca","Volumes":null,"WorkingDir":"","Entrypoint":null,"OnBuild":null,"Labels":{"maintainer":"Clement Verna \u003ccverna@fedoraproject.org\u003e"}},"container":"54e75e92a843080f28bf7128b765a3469d523d1f11c354b783926c36e0ff4696","container_config":{"Hostname":"54e75e92a843","Domainname":"","User":"","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin","DISTTAG=f33container","FGC=f33","FBR=f33"],"Cmd":["/bin/sh","-c","#(nop) ","CMD [\"/bin/bash\"]"],"Image":"sha256:2ad0e7e335b63ed128ee487e6140eb731c6202b9b1adf02da3c1a5a605028dca","Volumes":null,"WorkingDir":"","Entrypoint":null,"OnBuild":null,"Labels":{"maintainer":"Clement Verna \u003ccverna@fedoraproject.org\u003e"}},"created":"2021-01-26T00:23:51.73608945Z","docker_version":"19.03.12","history":[{"created":"2019-01-16T21:21:55.569693599Z","created_by":"/bin/sh -c #(nop)  LABEL maintainer=Clement Verna \u003ccverna@fedoraproject.org\u003e","empty_layer":true},{"created":"2020-04-30T23:21:44.324893962Z","created_by":"/bin/sh -c #(nop)  ENV DISTTAG=f33container FGC=f33 FBR=f33","empty_layer":true},{"created":"2021-01-26T00:23:51.35806501Z","created_by":"/bin/sh -c #(nop) ADD file:95aeb73fea2ac65cad5cf0046ca2e09ba4bf988e9c0cecdd816a199958a7cdb5 in / "},{"created":"2021-01-26T00:23:51.73608945Z","created_by":"/bin/sh -c #(nop)  CMD [\"/bin/bash\"]","empty_layer":true}],"os":"linux","rootfs":{"type":"layers","diff_ids":["sha256:5d6d8687c4a028c69d16dcd730084d6996490fd41556dbdc065ebac533204f2a"]}}"##;

pub(super) const DOCKER_IMAGE_MANIFEST_BLOB: &str = r##"{
   "schemaVersion": 2,
   "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
   "config": {
      "mediaType": "application/vnd.docker.container.image.v1+json",
      "size": 1994,
      "digest": "sha256:a78267678b7e6e849c7e960b09227b737a38d5073a5071b041a16bd4b609ef92"
   },
   "layers": [
      {
         "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
         "size": 63670669,
         "digest": "sha256:f147208a1e03a819ae351de51e039b1d5ba3fb18b09fe213dd04324149cc71e6"
      }
   ]
}"##;
