#  (2023-02-06) v0.7.0

### Bug Fixes

* Return `from_fn` ([5afd662](https://github.com/Pscheidl/enum-collections/commit/5afd662271d1ae68b377c613567c89ec70edcb28))

#  (2023-01-15) v0.6.0


### Bug Fixes

* Deallocate memory reserved for EnumMap's values on when dropped ([e3ee18b](https://github.com/Pscheidl/enum-map/commit/e3ee18b98e18b447ff15a5c78fbb770c1380dcc1))
* Drop memory allocated by EnumMap ([86e6d23](https://github.com/Pscheidl/enum-map/commit/86e6d231065fcdf0c372e4bd7a411fccfedba74e))
* Enum-collections local FS dependency + point to the current unreleased version ([2b6a07e](https://github.com/Pscheidl/enum-map/commit/2b6a07e59d44ab51e4f68cb4c95bc6bdca7a0691))


### Features

* disallow custom discriminants ([5925a72](https://github.com/Pscheidl/enum-map/commit/5925a722251a5f9bb00279e69191089e31182f2c))
* provide array of variants ([ce6bd67](https://github.com/Pscheidl/enum-map/commit/ce6bd679a4c4af023f1e5ea6b2a7e25867fc04ed))


#  (2023-01-04) v0.5.0

### Bug Fixes

* Deallocate memory reserved for EnumMap's values on when dropped ([e3ee18b](https://github.com/Pscheidl/enum-map/commit/e3ee18b98e18b447ff15a5c78fbb770c1380dcc1))
* Drop memory allocated by EnumMap ([86e6d23](https://github.com/Pscheidl/enum-map/commit/86e6d231065fcdf0c372e4bd7a411fccfedba74e))

#  (2022-12-30) v0.4.0


### Features

* Copy enum instead of creating a reference ([16d8736](https://github.com/Pscheidl/enum-map/commit/16d8736908b7db069e6bfd43d9ad0182cb96f335))
* derive(Enumerated) macro as an alternative to existing attribute macro ([038f4ab](https://github.com/Pscheidl/enum-map/commit/038f4abfc7a6b6adcd1b23bf7660b0ff7b62f512))

#  (2022-12-30) 0.1.0 - 0.3.0 


### Features

* Add `reset` operation to EnumTable ([2ef3d1d](https://github.com/Pscheidl/enum-map/commit/2ef3d1de9b9e2e4548bd0d2197d4eda3548b9018))
* Enummap macro ([7b1ef7d](https://github.com/Pscheidl/enum-map/commit/7b1ef7d3336f47a08511601f350e745603fe530e))
* EnumTable ([b96ad28](https://github.com/Pscheidl/enum-map/commit/b96ad2845dee5aab437fdc07ea2d9f93594102bd))
* GET benchmark ([293eb5b](https://github.com/Pscheidl/enum-map/commit/293eb5b0fba706e6d512458e65980ad6c3557d31))
* Impl Default for EnumMap ([79f08a9](https://github.com/Pscheidl/enum-map/commit/79f08a9bbed4fa44dc9aa895676d29efd45a8032))
* Implement `remove` operation for EnumMap ([3c42bd7](https://github.com/Pscheidl/enum-map/commit/3c42bd75a9551eb7229c93a3ef384e4ac8563edb))
* Implement Index and IndexMut for EnumMap ([03c05ca](https://github.com/Pscheidl/enum-map/commit/03c05ca8b5bc45b253c2fa05c0b96c3595318c00))
* Index and IndexMut for EnumTable ([66a0088](https://github.com/Pscheidl/enum-map/commit/66a0088755d347f91110853a7b31e52ff95ce2c5))
* Insert & Get ([4118964](https://github.com/Pscheidl/enum-map/commit/4118964fc5681023a7fccc7837ad38700773d490))
* Insert benchmarks ([1990e74](https://github.com/Pscheidl/enum-map/commit/1990e747715e543dba81bec20f9b512b84418006))


### Reverts

* Revert "chore: Remove enummap macros subproject" ([8a0d76a](https://github.com/Pscheidl/enum-map/commit/8a0d76a81c8fe11b2bb924c59429697a05e35594))



