# cargo `target_applies_to_host` effects


With a host of x86_64-unknown-linux-gnu
| | target_applies_to_host=true | target_applies_to_host=false |
|-|-|-|
| no --target flag | Flag passed to bin<br/>Flag passed to proc macro<br/> | Flag not passed to bin<br/>Flag not passed to proc macro<br/> |
| --target x86_64-unknown-linux-gnu | Flag passed to bin<br/>Flag not passed to proc macro<br/> | Flag passed to bin<br/>Flag not passed to proc macro<br/> |
| --target i686-unknown-linux-gnu | Flag passed to bin<br/>Flag not passed to proc macro<br/> | Flag passed to bin<br/>Flag not passed to proc macro<br/> |
