.bail on
.timer on
.load ./base64.dylib

select base64_encode("abc");

select base64_decode("eHl6");
