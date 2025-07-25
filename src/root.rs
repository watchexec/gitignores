
use crate::GitIgnore;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Root {
	#[cfg(feature = "root-actionscript")]
	Actionscript,
	#[cfg(feature = "root-ada")]
	Ada,
	#[cfg(feature = "root-adventure-game-studio")]
	AdventureGameStudio,
	#[cfg(feature = "root-agda")]
	Agda,
	#[cfg(feature = "root-al")]
	Al,
	#[cfg(feature = "root-android")]
	Android,
	#[cfg(feature = "root-angular")]
	Angular,
	#[cfg(feature = "root-app-engine")]
	AppEngine,
	#[cfg(feature = "root-appcelerator-titanium")]
	AppceleratorTitanium,
	#[cfg(feature = "root-arch-linux-packages")]
	ArchLinuxPackages,
	#[cfg(feature = "root-autotools")]
	Autotools,
	#[cfg(feature = "root-ballerina")]
	Ballerina,
	#[cfg(feature = "root-c")]
	C,
	#[cfg(feature = "root-c-make")]
	CMake,
	#[cfg(feature = "root-c-plus-plus")]
	CPlusPlus,
	#[cfg(feature = "root-cake-php")]
	CakePhp,
	#[cfg(feature = "root-cf-wheels")]
	CfWheels,
	#[cfg(feature = "root-chef-cookbook")]
	ChefCookbook,
	#[cfg(feature = "root-clojure")]
	Clojure,
	#[cfg(feature = "root-code-igniter")]
	CodeIgniter,
	#[cfg(feature = "root-common-lisp")]
	CommonLisp,
	#[cfg(feature = "root-composer")]
	Composer,
	#[cfg(feature = "root-concrete5")]
	Concrete5,
	#[cfg(feature = "root-coq")]
	Coq,
	#[cfg(feature = "root-craft-cms")]
	CraftCms,
	#[cfg(feature = "root-cuda")]
	Cuda,
	#[cfg(feature = "root-d")]
	D,
	#[cfg(feature = "root-dart")]
	Dart,
	#[cfg(feature = "root-delphi")]
	Delphi,
	#[cfg(feature = "root-dm")]
	Dm,
	#[cfg(feature = "root-dotnet")]
	Dotnet,
	#[cfg(feature = "root-drupal")]
	Drupal,
	#[cfg(feature = "root-e-pi-server")]
	EPiServer,
	#[cfg(feature = "root-eagle")]
	Eagle,
	#[cfg(feature = "root-ecutest")]
	Ecutest,
	#[cfg(feature = "root-elisp")]
	Elisp,
	#[cfg(feature = "root-elixir")]
	Elixir,
	#[cfg(feature = "root-elm")]
	Elm,
	#[cfg(feature = "root-erlang")]
	Erlang,
	#[cfg(feature = "root-expression-engine")]
	ExpressionEngine,
	#[cfg(feature = "root-ext-js")]
	ExtJs,
	#[cfg(feature = "root-fancy")]
	Fancy,
	#[cfg(feature = "root-finale")]
	Finale,
	#[cfg(feature = "root-firebase")]
	Firebase,
	#[cfg(feature = "root-flax-engine")]
	FlaxEngine,
	#[cfg(feature = "root-flutter")]
	Flutter,
	#[cfg(feature = "root-force-dot-com")]
	ForceDotCom,
	#[cfg(feature = "root-fortran")]
	Fortran,
	#[cfg(feature = "root-fuel-php")]
	FuelPhp,
	#[cfg(feature = "root-gcov")]
	Gcov,
	#[cfg(feature = "root-git-book")]
	GitBook,
	#[cfg(feature = "root-git-hub-pages")]
	GitHubPages,
	#[cfg(feature = "root-gleam")]
	Gleam,
	#[cfg(feature = "root-go")]
	Go,
	#[cfg(feature = "root-godot")]
	Godot,
	#[cfg(feature = "root-gradle")]
	Gradle,
	#[cfg(feature = "root-grails")]
	Grails,
	#[cfg(feature = "root-gwt")]
	Gwt,
	#[cfg(feature = "root-haskell")]
	Haskell,
	#[cfg(feature = "root-haxe")]
	Haxe,
	#[cfg(feature = "root-hip")]
	Hip,
	#[cfg(feature = "root-iar")]
	Iar,
	#[cfg(feature = "root-idris")]
	Idris,
	#[cfg(feature = "root-igor-pro")]
	IgorPro,
	#[cfg(feature = "root-j-boss")]
	JBoss,
	#[cfg(feature = "root-java")]
	Java,
	#[cfg(feature = "root-jekyll")]
	Jekyll,
	#[cfg(feature = "root-jenkins-home")]
	JenkinsHome,
	#[cfg(feature = "root-joomla")]
	Joomla,
	#[cfg(feature = "root-julia")]
	Julia,
	#[cfg(feature = "root-katalon")]
	Katalon,
	#[cfg(feature = "root-ki-cad")]
	KiCad,
	#[cfg(feature = "root-kohana")]
	Kohana,
	#[cfg(feature = "root-kotlin")]
	Kotlin,
	#[cfg(feature = "root-lab-view")]
	LabView,
	#[cfg(feature = "root-lang-chain")]
	LangChain,
	#[cfg(feature = "root-laravel")]
	Laravel,
	#[cfg(feature = "root-leiningen")]
	Leiningen,
	#[cfg(feature = "root-lemon-stand")]
	LemonStand,
	#[cfg(feature = "root-lilypond")]
	Lilypond,
	#[cfg(feature = "root-lithium")]
	Lithium,
	#[cfg(feature = "root-lua")]
	Lua,
	#[cfg(feature = "root-luau")]
	Luau,
	#[cfg(feature = "root-magento")]
	Magento,
	#[cfg(feature = "root-maven")]
	Maven,
	#[cfg(feature = "root-mercury")]
	Mercury,
	#[cfg(feature = "root-meta-programming-system")]
	MetaProgrammingSystem,
	#[cfg(feature = "root-modelica")]
	Modelica,
	#[cfg(feature = "root-nanoc")]
	Nanoc,
	#[cfg(feature = "root-nestjs")]
	Nestjs,
	#[cfg(feature = "root-nextjs")]
	Nextjs,
	#[cfg(feature = "root-nim")]
	Nim,
	#[cfg(feature = "root-nix")]
	Nix,
	#[cfg(feature = "root-node")]
	Node,
	#[cfg(feature = "root-o-caml")]
	OCaml,
	#[cfg(feature = "root-objective-c")]
	ObjectiveC,
	#[cfg(feature = "root-opa")]
	Opa,
	#[cfg(feature = "root-open-cart")]
	OpenCart,
	#[cfg(feature = "root-oracle-forms")]
	OracleForms,
	#[cfg(feature = "root-packer")]
	Packer,
	#[cfg(feature = "root-perl")]
	Perl,
	#[cfg(feature = "root-phalcon")]
	Phalcon,
	#[cfg(feature = "root-play-framework")]
	PlayFramework,
	#[cfg(feature = "root-plone")]
	Plone,
	#[cfg(feature = "root-prestashop")]
	Prestashop,
	#[cfg(feature = "root-processing")]
	Processing,
	#[cfg(feature = "root-pure-script")]
	PureScript,
	#[cfg(feature = "root-python")]
	Python,
	#[cfg(feature = "root-qooxdoo")]
	Qooxdoo,
	#[cfg(feature = "root-qt")]
	Qt,
	#[cfg(feature = "root-r")]
	R,
	#[cfg(feature = "root-racket")]
	Racket,
	#[cfg(feature = "root-rails")]
	Rails,
	#[cfg(feature = "root-raku")]
	Raku,
	#[cfg(feature = "root-re-script")]
	ReScript,
	#[cfg(feature = "root-rhodes-rhomobile")]
	RhodesRhomobile,
	#[cfg(feature = "root-ros")]
	Ros,
	#[cfg(feature = "root-ruby")]
	Ruby,
	#[cfg(feature = "root-rust")]
	Rust,
	#[cfg(feature = "root-s-cons")]
	SCons,
	#[cfg(feature = "root-sass")]
	Sass,
	#[cfg(feature = "root-scala")]
	Scala,
	#[cfg(feature = "root-scheme")]
	Scheme,
	#[cfg(feature = "root-scrivener")]
	Scrivener,
	#[cfg(feature = "root-sdcc")]
	Sdcc,
	#[cfg(feature = "root-seam-gen")]
	SeamGen,
	#[cfg(feature = "root-sketch-up")]
	SketchUp,
	#[cfg(feature = "root-smalltalk")]
	Smalltalk,
	#[cfg(feature = "root-solidity-remix")]
	SolidityRemix,
	#[cfg(feature = "root-ssdt-sqlproj")]
	SsdtSqlproj,
	#[cfg(feature = "root-stella")]
	Stella,
	#[cfg(feature = "root-sugar-crm")]
	SugarCrm,
	#[cfg(feature = "root-swift")]
	Swift,
	#[cfg(feature = "root-symfony")]
	Symfony,
	#[cfg(feature = "root-symphony-cms")]
	SymphonyCms,
	#[cfg(feature = "root-te-x")]
	TeX,
	#[cfg(feature = "root-terraform")]
	Terraform,
	#[cfg(feature = "root-test-complete")]
	TestComplete,
	#[cfg(feature = "root-textpattern")]
	Textpattern,
	#[cfg(feature = "root-turbo-gears2")]
	TurboGears2,
	#[cfg(feature = "root-twin-cat3")]
	TwinCat3,
	#[cfg(feature = "root-typo3")]
	Typo3,
	#[cfg(feature = "root-unity")]
	Unity,
	#[cfg(feature = "root-unreal-engine")]
	UnrealEngine,
	#[cfg(feature = "root-vba")]
	Vba,
	#[cfg(feature = "root-visual-studio")]
	VisualStudio,
	#[cfg(feature = "root-vvvv")]
	Vvvv,
	#[cfg(feature = "root-waf")]
	Waf,
	#[cfg(feature = "root-word-press")]
	WordPress,
	#[cfg(feature = "root-xojo")]
	Xojo,
	#[cfg(feature = "root-yeoman")]
	Yeoman,
	#[cfg(feature = "root-yii")]
	Yii,
	#[cfg(feature = "root-zend-framework")]
	ZendFramework,
	#[cfg(feature = "root-zephir")]
	Zephir,
	#[cfg(feature = "root-zig")]
	Zig,
}

impl GitIgnore for Root {
	#[cfg(feature = "no-contents")]
	fn contents(self) -> &'static str {
		""
	}

	#[cfg(not(feature = "no-contents"))]
	fn contents(self) -> &'static str {
		match self { #[cfg(feature = "root-actionscript")] Self::Actionscript => "# Build and Release Folders\nbin-debug/\nbin-release/\n[Oo]bj/\n[Bb]in/\n\n# Other files and folders\n.settings/\n\n# Executables\n*.swf\n*.air\n*.ipa\n*.apk\n\n# Project files, i.e. `.project`, `.actionScriptProperties` and `.flexProperties`\n# should NOT be excluded as they contain compiler settings and other important\n# information for Eclipse / Flash Builder.\n", #[cfg(feature = "root-ada")] Self::Ada => "# Object file\n*.o\n\n# Ada Library Information\n*.ali\n", #[cfg(feature = "root-adventure-game-studio")] Self::AdventureGameStudio => "# Built things\n_Debug/\nCompiled/\n\n# AudioCache can be rebuilt from sources\nAudioCache/\n\n# Lockfile\n_OpenInEditor.lock\n\n# User settings\nGame.agf.user\n*.crm.user\n\n# Backups\nGame.agf.bak\nbackup_acsprset.spr\n\n# Memory dumps\n*.dmp\n\n# Temporary files\n# temporarily created during sprite or room background compression\n~aclzw.tmp\n# temporary, main game data, before getting packed into exe\ngame28.dta\n# temporary build of the game before being moved to Compiled/ folder\n*.exe\n\n# Log files\nwarnings.log\n", #[cfg(feature = "root-agda")] Self::Agda => "*.agdai\nMAlonzo/**\n", #[cfg(feature = "root-al")] Self::Al => "### AL ###\n#Template for AL projects for Dynamics 365 Business Central\n#launch.json folder\n.vscode/\n#Cache folder\n.alcache/\n#Symbols folder\n.alpackages/\n#Snapshots folder\n.snapshots/\n#Testing Output folder\n.output/\n#Extension App-file\n*.app\n#Rapid Application Development File\nrad.json\n#Translation Base-file\n*.g.xlf\n#License-file\n*.flf\n#Test results file\nTestResults.xml", #[cfg(feature = "root-android")] Self::Android => "# Gradle files\n.gradle/\nbuild/\n\n# Local configuration file (sdk path, etc)\nlocal.properties\n\n# Log/OS Files\n*.log\n\n# Android Studio generated files and folders\ncaptures/\n.externalNativeBuild/\n.cxx/\n*.aab\n*.apk\noutput-metadata.json\n\n# IntelliJ\n*.iml\n.idea/\nmisc.xml\ndeploymentTargetDropDown.xml\nrender.experimental.xml\n\n# Keystore files\n*.jks\n*.keystore\n\n# Google Services (e.g. APIs or Firebase)\ngoogle-services.json\n\n# Android Profiling\n*.hprof\n", #[cfg(feature = "root-angular")] Self::Angular => "# Angular specific\n/dist/\n/out-tsc/\n/tmp/\n/coverage/\n/e2e/test-output/\n/.angular/\n.angular/\n\n# Node modules and dependency files\n/node_modules/\n/package-lock.json\n/yarn.lock\n\n# Environment files\n/.env\n\n# Angular CLI and build artefacts\n/.angular-cli.json\n/.ng/\n\n# TypeScript cache\n*.tsbuildinfo\n\n# Logs\nnpm-debug.log*\nyarn-debug.log*\nyarn-error.log*\n", #[cfg(feature = "root-app-engine")] Self::AppEngine => "# Google App Engine generated folder\nappengine-generated/\n", #[cfg(feature = "root-appcelerator-titanium")] Self::AppceleratorTitanium => "# Build folder and log file\nbuild/\nbuild.log\n", #[cfg(feature = "root-arch-linux-packages")] Self::ArchLinuxPackages => "*.tar\n*.tar.*\n*.jar\n*.exe\n*.msi\n*.deb\n*.zip\n*.tgz\n*.log\n*.log.*\n*.sig\n\npkg/\nsrc/\n", #[cfg(feature = "root-autotools")] Self::Autotools => "# http://www.gnu.org/software/automake\n\nMakefile.in\n/ar-lib\n/mdate-sh\n/py-compile\n/test-driver\n/ylwrap\n.deps/\n.dirstamp\n\n# http://www.gnu.org/software/autoconf\n\nautom4te.cache\n/autoscan.log\n/autoscan-*.log\n/aclocal.m4\n/compile\n/config.cache\n/config.guess\n/config.h.in\n/config.log\n/config.status\n/config.sub\n/configure\n/configure.scan\n/depcomp\n/install-sh\n/missing\n/stamp-h1\n\n# https://www.gnu.org/software/libtool/\n\n/libtool\n/ltmain.sh\n.libs/\n\n# http://www.gnu.org/software/texinfo\n\n/texinfo.tex\n\n# http://www.gnu.org/software/m4/\n\nm4/libtool.m4\nm4/ltoptions.m4\nm4/ltsugar.m4\nm4/ltversion.m4\nm4/lt~obsolete.m4\n\n# Generated Makefile\n# (meta build system like autotools,\n# can automatically generate from config.status script\n# (which is called by configure script))\nMakefile\n", #[cfg(feature = "root-ballerina")] Self::Ballerina => "# generated files\ntarget/\ngenerated/\n\n# dependencies\nDependencies.toml\n\n# config files\nConfig.toml\n# the config files used for testing, Uncomment the following line if you want to commit the test config files\n#!**/tests/Config.toml\n", #[cfg(feature = "root-c")] Self::C => "# Prerequisites\n*.d\n\n# Object files\n*.o\n*.ko\n*.obj\n*.elf\n\n# Linker output\n*.ilk\n*.map\n*.exp\n\n# Precompiled Headers\n*.gch\n*.pch\n\n# Libraries\n*.lib\n*.a\n*.la\n*.lo\n\n# Shared objects (inc. Windows DLLs)\n*.dll\n*.so\n*.so.*\n*.dylib\n\n# Executables\n*.exe\n*.out\n*.app\n*.i*86\n*.x86_64\n*.hex\n\n# Debug files\n*.dSYM/\n*.su\n*.idb\n*.pdb\n\n# Kernel Module Compile Results\n*.mod*\n*.cmd\n.tmp_versions/\nmodules.order\nModule.symvers\nMkfile.old\ndkms.conf\n\n# debug information files\n*.dwo\n", #[cfg(feature = "root-c-make")] Self::CMake => "CMakeLists.txt.user\nCMakeCache.txt\nCMakeFiles\nCMakeScripts\nTesting\nMakefile\ncmake_install.cmake\ninstall_manifest.txt\ncompile_commands.json\nCTestTestfile.cmake\n_deps\nCMakeUserPresets.json\n\n# CLion\n#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can\n#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore\n#  and can be added to the global gitignore or merged into this file.  For a more nuclear\n#  option (not recommended) you can uncomment the following to ignore the entire idea folder.\n#cmake-build-*\n", #[cfg(feature = "root-c-plus-plus")] Self::CPlusPlus => "# Prerequisites\n*.d\n\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj\n\n# Precompiled Headers\n*.gch\n*.pch\n\n# Linker files\n*.ilk\n\n# Debugger Files\n*.pdb\n\n# Compiled Dynamic libraries\n*.so\n*.dylib\n*.dll\n\n# Fortran module files\n*.mod\n*.smod\n\n# Compiled Static libraries\n*.lai\n*.la\n*.a\n*.lib\n\n# Executables\n*.exe\n*.out\n*.app\n\n# debug information files\n*.dwo\n", #[cfg(feature = "root-cake-php")] Self::CakePhp => "# CakePHP 3\n\n/vendor/*\n/config/app.php\n\n/tmp/cache/models/*\n!/tmp/cache/models/empty\n/tmp/cache/persistent/*\n!/tmp/cache/persistent/empty\n/tmp/cache/views/*\n!/tmp/cache/views/empty\n/tmp/sessions/*\n!/tmp/sessions/empty\n/tmp/tests/*\n!/tmp/tests/empty\n\n/logs/*\n!/logs/empty\n\n# CakePHP 2\n\n/app/tmp/*\n/app/Config/core.php\n/app/Config/database.php\n/vendors/*\n", #[cfg(feature = "root-cf-wheels")] Self::CfWheels => "# unpacked plugin folders\nplugins/**/*\n\n# files directory where uploads go\nfiles\n\n# DBMigrate plugin: generated SQL\ndb/sql\n\n# AssetBundler plugin: generated bundles\njavascripts/bundles\nstylesheets/bundles\n", #[cfg(feature = "root-chef-cookbook")] Self::ChefCookbook => ".vagrant\n/cookbooks\n\n# Bundler\nbin/*\n.bundle/*\n\n.kitchen/\n.kitchen.local.yml\n", #[cfg(feature = "root-clojure")] Self::Clojure => "pom.xml\npom.xml.asc\n*.jar\n*.class\n/lib/\n/classes/\n/target/\n/checkouts/\n.lein-deps-sum\n.lein-repl-history\n.lein-plugins/\n.lein-failures\n.nrepl-port\n.cpcache/\n", #[cfg(feature = "root-code-igniter")] Self::CodeIgniter => "*/config/development\n*/logs/log-*.php\n!*/logs/index.html\n*/cache/*\n!system/cache/*\n!*/cache/index.html\n!*/cache/.htaccess\n\nuser_guide_src/build/*\nuser_guide_src/cilexer/build/*\nuser_guide_src/cilexer/dist/*\nuser_guide_src/cilexer/pycilexer.egg-info/*\n\n#codeigniter 3\napplication/logs/*\n!application/logs/index.html\n!application/logs/.htaccess\n/vendor/\n", #[cfg(feature = "root-common-lisp")] Self::CommonLisp => "*.FASL\n*.fasl\n*.lisp-temp\n*.dfsl\n*.pfsl\n*.d64fsl\n*.p64fsl\n*.lx64fsl\n*.lx32fsl\n*.dx64fsl\n*.dx32fsl\n*.fx64fsl\n*.fx32fsl\n*.sx64fsl\n*.sx32fsl\n*.wx64fsl\n*.wx32fsl\n", #[cfg(feature = "root-composer")] Self::Composer => "composer.phar\n/vendor/\n\n# Commit your application's lock file https://getcomposer.org/doc/01-basic-usage.md#commit-your-composer-lock-file-to-version-control\n# You may choose to ignore a library lock file http://getcomposer.org/doc/02-libraries.md#lock-file\n# composer.lock\n", #[cfg(feature = "root-concrete5")] Self::Concrete5 => "# ignore the error log and .htaccess and others\nerror_log\n.htaccess\n\n# concrete5 5.6 specific\n\nconfig/site.php\nfiles/cache/*\nfiles/tmp/*\n\n# concrete5 5.7 specific\n\n# ignore everything but the index.html\n/application/files/*\n!/application/files/index.html\n\n# ignore updates folder\n/updates/*\n\n# ignore sitemap.xml\n/sitemap.xml\n", #[cfg(feature = "root-coq")] Self::Coq => ".*.aux\n.*.d\n*.a\n*.cma\n*.cmi\n*.cmo\n*.cmx\n*.cmxa\n*.cmxs\n*.glob\n*.ml.d\n*.ml4.d\n*.mlg.d\n*.mli.d\n*.mllib.d\n*.mlpack.d\n*.native\n*.o\n*.v.d\n*.vio\n*.vo\n*.vok\n*.vos\n.coq-native\n.csdp.cache\n.lia.cache\n.nia.cache\n.nlia.cache\n.nra.cache\ncsdp.cache\nlia.cache\nnia.cache\nnlia.cache\nnra.cache\nnative_compute_profile_*.data\n\n# generated timing files\n*.timing.diff\n*.v.after-timing\n*.v.before-timing\n*.v.timing\ntime-of-build-after.log\ntime-of-build-before.log\ntime-of-build-both.log\ntime-of-build-pretty.log\n", #[cfg(feature = "root-craft-cms")] Self::CraftCms => "# Craft 2 Storage (https://craftcms.com/support/craft-storage-gitignore)\n# not necessary for Craft 3 (https://github.com/craftcms/craft/issues/26)\n/craft/storage/*\n!/craft/storage/rebrand\n", #[cfg(feature = "root-cuda")] Self::Cuda => "*.i\n*.ii\n*.gpu\n*.ptx\n*.cubin\n*.fatbin\n", #[cfg(feature = "root-d")] Self::D => "# Compiled Object files\n*.o\n*.obj\n\n# Compiled Dynamic libraries\n*.so\n*.dylib\n*.dll\n\n# Compiled Static libraries\n*.a\n*.lib\n\n# Executables\n*.exe\n\n# DUB\n.dub\ndocs.json\n__dummy.html\ndocs/\n\n# Code coverage\n*.lst\n", #[cfg(feature = "root-dart")] Self::Dart => "# See https://www.dartlang.org/guides/libraries/private-files\n\n# Files and directories created by pub\n.dart_tool/\n.packages\nbuild/\n# If you're building an application, you may want to check-in your pubspec.lock\npubspec.lock\n\n# Directory created by dartdoc\n# If you don't generate documentation locally you can remove this line.\ndoc/api/\n\n# dotenv environment variables file\n.env*\n\n# Avoid committing generated Javascript files:\n*.dart.js\n# Produced by the --dump-info flag.\n*.info.json\n# When generated by dart2js. Don't specify *.js if your\n# project includes source files written in JavaScript.\n*.js\n*.js_\n*.js.deps\n*.js.map\n\n.flutter-plugins\n.flutter-plugins-dependencies\n", #[cfg(feature = "root-delphi")] Self::Delphi => "# Uncomment these types if you want even more clean repository. But be careful.\n# It can make harm to an existing project source. Read explanations below.\n#\n# Resource files are binaries containing manifest, project icon and version info.\n# They can not be viewed as text or compared by diff-tools. Consider replacing them with .rc files.\n#*.res\n#\n# Type library file (binary). In old Delphi versions it should be stored.\n# Since Delphi 2009 it is produced from .ridl file and can safely be ignored.\n#*.tlb\n#\n# Diagram Portfolio file. Used by the diagram editor up to Delphi 7.\n# Uncomment this if you are not using diagrams or use newer Delphi version.\n#*.ddp\n#\n# Visual LiveBindings file. Added in Delphi XE2.\n# Uncomment this if you are not using LiveBindings Designer.\n#*.vlb\n#\n# Deployment Manager configuration file for your project. Added in Delphi XE2.\n# Uncomment this if it is not mobile development and you do not use remote debug feature.\n#*.deployproj\n#\n# C++ object files produced when C/C++ Output file generation is configured.\n# Uncomment this if you are not using external objects (zlib library for example).\n#*.obj\n#\n\n# Default Delphi compiler directories\n# Content of this directories are generated with each Compile/Construct of a project.\n# Most of the time, files here have not there place in a code repository.\n#Win32/\n#Win64/\n#OSX64/\n#OSXARM64/\n#Android/\n#Android64/\n#iOSDevice64/\n#Linux64/\n\n# Delphi compiler-generated binaries (safe to delete)\n*.exe\n*.dll\n*.bpl\n*.bpi\n*.dcp\n*.so\n*.apk\n*.drc\n*.map\n*.dres\n*.rsm\n*.tds\n*.dcu\n*.lib\n*.a\n*.o\n*.ocx\n\n# Delphi autogenerated files (duplicated info)\n*.cfg\n*.hpp\n*Resource.rc\n\n# Delphi local files (user-specific info)\n*.local\n*.identcache\n*.projdata\n*.tvsconfig\n*.dsk\n*.dsv\n\n# Delphi history and backups\n__history/\n__recovery/\n*.~*\n\n# Castalia statistics file (since XE7 Castalia is distributed with Delphi)\n*.stat\n\n# Boss dependency manager vendor folder https://github.com/HashLoad/boss\nmodules/\n", #[cfg(feature = "root-dm")] Self::Dm => "*.dmb\n*.rsc\n*.int\n*.lk\n*.zip\n", #[cfg(feature = "root-dotnet")] Self::Dotnet => "## A streamlined .gitignore for modern .NET projects\n## including temporary files, build results, and\n## files generated by popular .NET tools. If you are\n## developing with Visual Studio, the VS .gitignore\n## https://github.com/github/gitignore/blob/main/VisualStudio.gitignore\n## has more thorough IDE-specific entries.\n##\n## Get latest from https://github.com/github/gitignore/blob/main/Dotnet.gitignore\n\n# Build results\n[Dd]ebug/\n[Dd]ebugPublic/\n[Rr]elease/\n[Rr]eleases/\nx64/\nx86/\n[Ww][Ii][Nn]32/\n[Aa][Rr][Mm]/\n[Aa][Rr][Mm]64/\nbld/\n[Bb]in/\n[Oo]bj/\n[Ll]og/\n[Ll]ogs/\n\n# .NET Core\nproject.lock.json\nproject.fragment.lock.json\nartifacts/\n\n# ASP.NET Scaffolding\nScaffoldingReadMe.txt\n\n# NuGet Packages\n*.nupkg\n# NuGet Symbol Packages\n*.snupkg\n\n# Others\n~$*\n*~\nCodeCoverage/\n\n# MSBuild Binary and Structured Log\n*.binlog\n\n# MSTest test Results\n[Tt]est[Rr]esult*/\n[Bb]uild[Ll]og.*\n\n# NUnit\n*.VisualState.xml\nTestResult.xml\nnunit-*.xml", #[cfg(feature = "root-drupal")] Self::Drupal => "# gitignore template for Drupal 8 projects\n#\n# earlier versions of Drupal are tracked in `community/PHP/`\n#\n# follows official upstream conventions:\n# https://www.drupal.org/docs/develop/using-composer\n\n# Ignore configuration files that may contain sensitive information\n/web/sites/*/*settings*.php\n/web/sites/*/*services*.yml\n\n# Ignore paths that may contain user-generated content\n/web/sites/*/files\n/web/sites/*/public\n/web/sites/*/private\n/web/sites/*/files-public\n/web/sites/*/files-private\n\n# Ignore paths that may contain temporary files\n/web/sites/*/translations\n/web/sites/*/tmp\n/web/sites/*/cache\n\n# Ignore drupal core (if not versioning drupal sources)\n/web/vendor\n/web/core\n/web/modules/README.txt\n/web/modules/contrib\n/web/profiles/README.txt\n/web/profiles/contrib\n/web/sites/development.services.yml\n/web/sites/example.settings.local.php\n/web/sites/example.sites.php\n/web/sites/README.txt\n/web/themes/README.txt\n/web/themes/contrib\n/web/.csslintrc\n/web/.editorconfig\n/web/.eslintignore\n/web/.eslintrc.json\n/web/.gitattributes\n/web/.htaccess\n/web/.ht.router.php\n/web/autoload.php\n/web/composer.json\n/web/composer.lock\n/web/example.gitignore\n/web/index.php\n/web/INSTALL.txt\n/web/LICENSE.txt\n/web/README.txt\n/web/robots.txt\n/web/update.php\n/web/web.config\n\n# Ignore vendor dependencies and scripts\n/vendor\n/composer.phar\n/composer\n/robo.phar\n/robo\n/drush.phar\n/drush\n/drupal.phar\n/drupal\n", #[cfg(feature = "root-e-pi-server")] Self::EPiServer => "######################\n## EPiServer Files\n######################\n*License.config\n", #[cfg(feature = "root-eagle")] Self::Eagle => "# Ignore list for Eagle, a PCB layout tool\n\n# Backup files\n*.s#?\n*.b#?\n*.l#?\n*.b$?\n*.s$?\n*.l$?\n\n# Eagle project file\n# It contains a serial number and references to the file structure\n# on your computer.\n# comment the following line if you want to have your project file included.\neagle.epf\n\n# Autorouter files\n*.pro\n*.job\n\n# CAM files\n*.$$$\n*.cmp\n*.ly2\n*.l15\n*.sol\n*.plc\n*.stc\n*.sts\n*.crc\n*.crs\n\n*.dri\n*.drl\n*.gpi\n*.pls\n*.ger\n*.xln\n\n*.drd\n*.drd.*\n\n*.s#*\n*.b#*\n\n*.info\n\n*.eps\n\n# file locks introduced since 7.x\n*.lck\n", #[cfg(feature = "root-ecutest")] Self::Ecutest => "# gitignore template for ecu.test workspaces - by tracetronic https://tracetronic.com\n# website: https://www.ecu-test.com\n#   * all directories are related to the default directories, please adapt the .gitignore if you use customized directories\n\n# Dynamic workspace settings\n#   * We don't recommend to ignore the .workspace directory, because of important \n#     * project specific settings\n#     * local user settings\n.workspace/ETdrive.xml\n.workspace/favorites.xml\n.workspace/filters.xml\n.workspace/generators.xml\n.workspace/history.xml\n.workspace/parallelExecution.xml\n.workspace/signalviewer.xml\n.workspace/signalViewerHistory.json\n.workspace/signalviewer2layout.xml\n.workspace/testeditor.xml\n.workspace/tooladapter.xml\n.workspace/view.xml\n# optional, if your process depends on this file remove exclusion\n.workspace/attributeLists.xml\n.workspace/interactiveexecution.xml\n.workspace/protocol.xml\n.workspace/pythonlibrary.xml\n# deprecated, support for older versions\n.workspace/traceexplorer.xml\n\n# Custom file formats and test dependencies\n#  * you can manage your artifacts also with test.guide (https://www.test-guide.info) and reference them via Playbooks\n*.arxml\n*.a2l\n*.dbc\n*.hex\n*.s19\n[tT]estdata\n[tT]estdaten\n\n# Test results and test execution related content\n#   * Git is not intended to store and provide test results for all iterations\n#   * We recommend to use test.guide (https://www.test-guide.info) for the test report management\nTestReports\n\n# Report generators and templates\n#   * if you want to provide (f.e.) your own report generators exclude the directory here and ignore only the unnecessary subdirectories\nTemplates\n\n# optional, default for external Python libraries\nPyLibs\n\n# Exclude large binary artifacts\n#  * you can manage your artifacts also with test.guide (https://www.test-guide.info) and reference them via Playbooks\nOffline-FIUs\nOffline-Models\nOffline-SGBDs\n*.exe\n*.msi\n*.zip\n*.7z\n\n# Exclude default and custom temporary directories\nBackup_*\n\n# Python bytecode and cache files\n__pycache__/\n*.py[cod]\n", #[cfg(feature = "root-elisp")] Self::Elisp => "# Compiled\n*.elc\n\n# Packaging\n.cask/\n.eask/\n.eldev/\n.keg/\n\n# Built distribution\ndist/\n\n# Backup files\n*~\n\n# Undo-tree save-files\n*.~undo-tree\n", #[cfg(feature = "root-elixir")] Self::Elixir => "/_build\n/cover\n/deps\n/doc\n/.fetch\nerl_crash.dump\n*.ez\n*.beam\n/config/*.secret.exs\n.elixir_ls/\n", #[cfg(feature = "root-elm")] Self::Elm => "# elm-package generated files\nelm-stuff\n# elm-repl generated files\nrepl-temp-*\n", #[cfg(feature = "root-erlang")] Self::Erlang => ".eunit\n*.o\n*.beam\n*.plt\nerl_crash.dump\n.concrete/DEV_MODE\n\n# rebar 2.x\n.rebar\nrel/example_project\nebin/*.beam\ndeps\n\n# rebar 3\n.rebar3\n_build/\n_checkouts/\n", #[cfg(feature = "root-expression-engine")] Self::ExpressionEngine => ".DS_Store\n\n# Images\nimages/avatars/\nimages/captchas/\nimages/smileys/\nimages/member_photos/\nimages/signature_attachments/\nimages/pm_attachments/\n\n# For security do not publish the following files\nsystem/expressionengine/config/database.php\nsystem/expressionengine/config/config.php\n\n# Caches\nsized/\nthumbs/\n_thumbs/\n*/expressionengine/cache/*\n", #[cfg(feature = "root-ext-js")] Self::ExtJs => ".architect\nbootstrap.css\nbootstrap.js\nbootstrap.json\nbootstrap.jsonp\nbuild/\nclassic.json\nclassic.jsonp\next/\nmodern.json\nmodern.jsonp\nresources/sass/.sass-cache/\nresources/.arch-internal-preview.css\n.arch-internal-preview.css\n", #[cfg(feature = "root-fancy")] Self::Fancy => "*.rbc\n*.fyc\n", #[cfg(feature = "root-finale")] Self::Finale => "*.bak\n*.db\n*.avi\n*.pdf\n*.ps\n*.mid\n*.midi\n*.mp3\n*.aif\n*.wav\n# Some versions of Finale have a bug and randomly save extra copies of\n# the music source as \"<Filename> copy.mus\"\n*copy.mus\n", #[cfg(feature = "root-firebase")] Self::Firebase => "# Firebase build and deployment files\n/firebase-debug.log\n/firebase-debug.*.log\n.firebaserc\n\n# Firebase Hosting\n/firebase.json\n*.cache\nhosting/.cache\n\n# Firebase Functions\n/functions/node_modules/\n/functions/.env\n/functions/package-lock.json\n\n# Firebase Emulators\n/firebase-*.zip\n/.firebase/\n/emulator-ui/\n\n# Logs\n*.log\nnpm-debug.log*\nyarn-debug.log*\nyarn-error.log*\n\n# Environment files (local configs)\n/.env.*\n", #[cfg(feature = "root-flax-engine")] Self::FlaxEngine => "# Ignore Flax project files\nBinaries/\nCache/\nLogs/\nOutput/\nScreenshots/\n*.HotReload.*\n\n# Ignore Visual Studio project files (generated locally)\n*.csproj\n*.sln\n\n# Ignore thumbnails created by Windows\nThumbs.db\n\n# Ignore files built by Visual Studio\n*.obj\n*.exe\n*.pdb\n*.user\n*.aps\n*.pch\n*.vspscc\n*_i.c\n*_p.c\n*.ncb\n*.suo\n*.tlb\n*.tlh\n*.bak\n*.cache\n*.ilk\n*.log\n[Bb]in\n[Dd]ebug*/\n*.lib\n*.sbr\nobj/\n[Rr]elease*/\n_ReSharper*/\n[Tt]est[Rr]esult*\n.vs/\n\n# Ignore Nuget packages folder\npackages/\n", #[cfg(feature = "root-flutter")] Self::Flutter => "# Miscellaneous\n*.class\n*.lock\n*.log\n*.pyc\n*.swp\n.buildlog/\n.history\n\n\n\n# Flutter repo-specific\n/bin/cache/\n/bin/internal/bootstrap.bat\n/bin/internal/bootstrap.sh\n/bin/mingit/\n/dev/benchmarks/mega_gallery/\n/dev/bots/.recipe_deps\n/dev/bots/android_tools/\n/dev/devicelab/ABresults*.json\n/dev/docs/doc/\n/dev/docs/flutter.docs.zip\n/dev/docs/lib/\n/dev/docs/pubspec.yaml\n/dev/integration_tests/**/xcuserdata\n/dev/integration_tests/**/Pods\n/packages/flutter/coverage/\nversion\nanalysis_benchmark.json\n\n# packages file containing multi-root paths\n.packages.generated\n\n# Flutter/Dart/Pub related\n**/doc/api/\n.dart_tool/\n.flutter-plugins\n.flutter-plugins-dependencies\n**/generated_plugin_registrant.dart\n.packages\n.pub-preload-cache/\n.pub/\nbuild/\nflutter_*.png\nlinked_*.ds\nunlinked.ds\nunlinked_spec.ds\n\n# Android related\n**/android/**/gradle-wrapper.jar\n.gradle/\n**/android/captures/\n**/android/gradlew\n**/android/gradlew.bat\n**/android/local.properties\n**/android/**/GeneratedPluginRegistrant.java\n**/android/key.properties\n*.jks\n\n# iOS/XCode related\n**/ios/**/*.mode1v3\n**/ios/**/*.mode2v3\n**/ios/**/*.moved-aside\n**/ios/**/*.pbxuser\n**/ios/**/*.perspectivev3\n**/ios/**/*sync/\n**/ios/**/.sconsign.dblite\n**/ios/**/.tags*\n**/ios/**/.vagrant/\n**/ios/**/DerivedData/\n**/ios/**/Icon?\n**/ios/**/Pods/\n**/ios/**/.symlinks/\n**/ios/**/profile\n**/ios/**/xcuserdata\n**/ios/.generated/\n**/ios/Flutter/.last_build_id\n**/ios/Flutter/App.framework\n**/ios/Flutter/Flutter.framework\n**/ios/Flutter/Flutter.podspec\n**/ios/Flutter/Generated.xcconfig\n**/ios/Flutter/ephemeral\n**/ios/Flutter/app.flx\n**/ios/Flutter/app.zip\n**/ios/Flutter/flutter_assets/\n**/ios/Flutter/flutter_export_environment.sh\n**/ios/ServiceDefinitions.json\n**/ios/Runner/GeneratedPluginRegistrant.*\n\n# macOS\n**/Flutter/ephemeral/\n**/Pods/\n**/macos/Flutter/GeneratedPluginRegistrant.swift\n**/macos/Flutter/ephemeral\n**/xcuserdata/\n\n# Windows\n**/windows/flutter/generated_plugin_registrant.cc\n**/windows/flutter/generated_plugin_registrant.h\n**/windows/flutter/generated_plugins.cmake\n\n# Linux\n**/linux/flutter/generated_plugin_registrant.cc\n**/linux/flutter/generated_plugin_registrant.h\n**/linux/flutter/generated_plugins.cmake\n\n# Coverage\ncoverage/\n\n# Symbols\napp.*.symbols\n\n# Exceptions to above rules.\n!**/ios/**/default.mode1v3\n!**/ios/**/default.mode2v3\n!**/ios/**/default.pbxuser\n!**/ios/**/default.perspectivev3\n!/packages/flutter_tools/test/data/dart_dependencies_test/**/.packages\n!/dev/ci/**/Gemfile.lock", #[cfg(feature = "root-force-dot-com")] Self::ForceDotCom => ".project\n.settings\nsalesforce.schema\nReferenced Packages\n", #[cfg(feature = "root-fortran")] Self::Fortran => "# Prerequisites\n*.d\n\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj\n\n# Precompiled Headers\n*.gch\n*.pch\n\n# Linker files\n*.ilk\n\n# Debugger Files\n*.pdb\n\n# Compiled Dynamic libraries\n*.so\n*.dylib\n*.dll\n\n# Fortran module files\n*.mod\n*.smod\n\n# Compiled Static libraries\n*.lai\n*.la\n*.a\n*.lib\n\n# Executables\n*.exe\n*.out\n*.app\n\n# debug information files\n*.dwo\n", #[cfg(feature = "root-fuel-php")] Self::FuelPhp => "# the composer package lock file and install directory\n# Commit your application's lock file http://getcomposer.org/doc/01-basic-usage.md#composer-lock-the-lock-file\n# You may choose to ignore a library lock file http://getcomposer.org/doc/02-libraries.md#lock-file\n# /composer.lock\n/fuel/vendor\n\n# the fuelphp document\n/docs/\n\n# you may install these packages with `oil package`.\n# http://fuelphp.com/docs/packages/oil/package.html\n# /fuel/packages/auth/\n# /fuel/packages/email/\n# /fuel/packages/oil/\n# /fuel/packages/orm/\n# /fuel/packages/parser/\n\n# dynamically generated files\n/fuel/app/logs/*/*/*\n/fuel/app/cache/*/*\n/fuel/app/config/crypt.php\n", #[cfg(feature = "root-gcov")] Self::Gcov => "# gcc coverage testing tool files\n\n*.gcno\n*.gcda\n*.gcov\n", #[cfg(feature = "root-git-book")] Self::GitBook => "# Node rules:\n## Grunt intermediate storage (http://gruntjs.com/creating-plugins#storing-task-files)\n.grunt\n\n## Dependency directory\n## Commenting this out is preferred by some people, see\n## https://docs.npmjs.com/misc/faq#should-i-check-my-node_modules-folder-into-git\nnode_modules\n\n# Book build output\n_book\n\n# eBook build output\n*.epub\n*.mobi\n*.pdf\n", #[cfg(feature = "root-git-hub-pages")] Self::GitHubPages => "# This .gitignore is appropriate for repositories deployed to GitHub Pages and using\n# a Gemfile as specified at https://github.com/github/pages-gem#conventional\n\n# Basic Jekyll gitignores (synchronize to Jekyll.gitignore)\n_site/\n.sass-cache/\n.jekyll-cache/\n.jekyll-metadata\n\n# Additional Ruby/bundler ignore for when you run: bundle install\n/vendor\n\n# Specific ignore for GitHub Pages\n# GitHub Pages will always use its own deployed version of pages-gem \n# This means GitHub Pages will NOT use your Gemfile.lock and therefore it is\n# counterproductive to check this file into the repository.\n# Details at https://github.com/github/pages-gem/issues/768\nGemfile.lock\n", #[cfg(feature = "root-gleam")] Self::Gleam => "*.beam\n*.ez\n/build\nerl_crash.dump\n", #[cfg(feature = "root-go")] Self::Go => "# If you prefer the allow list template instead of the deny list, see community template:\n# https://github.com/github/gitignore/blob/main/community/Golang/Go.AllowList.gitignore\n#\n# Binaries for programs and plugins\n*.exe\n*.exe~\n*.dll\n*.so\n*.dylib\n\n# Test binary, built with `go test -c`\n*.test\n\n# Code coverage profiles and other test artifacts\n*.out\ncoverage.*\n*.coverprofile\nprofile.cov\n\n# Dependency directories (remove the comment below to include it)\n# vendor/\n\n# Go workspace file\ngo.work\ngo.work.sum\n\n# env file\n.env\n\n# Editor/IDE\n# .idea/\n# .vscode/\n", #[cfg(feature = "root-godot")] Self::Godot => "# Godot 4+ specific ignores\n.godot/\n.nomedia\n\n# Godot-specific ignores\n.import/\nexport.cfg\nexport_credentials.cfg\n\n# Imported translations (automatically generated from CSV files)\n*.translation\n\n# Mono-specific ignores\n.mono/\ndata_*/\nmono_crash.*.json\n", #[cfg(feature = "root-gradle")] Self::Gradle => ".gradle\n**/build/\n!**/src/**/build/\n\n# Ignore Gradle GUI config\ngradle-app.setting\n\n# Avoid ignoring Gradle wrapper jar file (.jar files are usually ignored)\n!gradle-wrapper.jar\n\n# Avoid ignore Gradle wrappper properties\n!gradle-wrapper.properties\n\n# Cache of project\n.gradletasknamecache\n\n# Eclipse Gradle plugin generated files\n# Eclipse Core\n.project\n# JDT-specific (Eclipse Java Development Tools)\n.classpath\n", #[cfg(feature = "root-grails")] Self::Grails => "# .gitignore for Grails 1.2 and 1.3\n# Although this should work for most versions of grails, it is\n# suggested that you use the \"grails integrate-with --git\" command\n# to generate your .gitignore file.\n\n# web application files\n/web-app/WEB-INF/classes\n\n# default HSQL database files for production mode\n/prodDb.*\n\n# general HSQL database files\n*Db.properties\n*Db.script\n\n# logs\n/stacktrace.log\n/test/reports\n/logs\n\n# project release file\n/*.war\n\n# plugin release files\n/*.zip\n/plugin.xml\n\n# older plugin install locations\n/plugins\n/web-app/plugins\n\n# \"temporary\" build files\n/target\n", #[cfg(feature = "root-gwt")] Self::Gwt => "*.class\n\n# Package Files #\n*.jar\n*.war\n\n# gwt caches and compiled units #\nwar/gwt_bree/\ngwt-unitCache/\n\n# boilerplate generated classes #\n.apt_generated/\n\n# more caches and things from deploy #\nwar/WEB-INF/deploy/\nwar/WEB-INF/classes/\n\n#compilation logs\n.gwt/\n\n#gwt junit compilation files\nwww-test/\n\n#old GWT (1.5) created this dir\n.gwt-tmp/\n", #[cfg(feature = "root-haskell")] Self::Haskell => "dist\ndist-*\ncabal-dev\n*.o\n*.hi\n*.hie\n*.chi\n*.chs.h\n*.dyn_o\n*.dyn_hi\n.hpc\n.hsenv\n.cabal-sandbox/\ncabal.sandbox.config\n*.prof\n*.aux\n*.hp\n*.eventlog\n.stack-work/\ncabal.project.local\ncabal.project.local~\n.HTF/\n.ghc.environment.*\n", #[cfg(feature = "root-haxe")] Self::Haxe => ".haxelib/\n.haxelsp/recording/\ndump/\n", #[cfg(feature = "root-hip")] Self::Hip => "# HIP.gitignore\n# GitHub gitignore template for AMD HIP (ROCm) projects\n#\n# Reference:\n#   Official AMD ROCm HIP .gitignore: https://github.com/ROCm/hip/blob/amd-staging/.gitignore\n\n# 1. Build directories and files\n/build/                          # common build directory\n/CMakeFiles/                     # CMake internal files\n/CMakeCache.txt                  # CMake cache file\n/Makefile                        # autogenerated Makefile\n/cmake_install.cmake             # install script\n/install_manifest.txt            # install manifest list\n*.ninja-dep                      # Ninja dependency files\n*.ninja_log                      # Ninja log files\nmeson-logs/                      # Meson log directory\n\n# 2. Compilation outputs and intermediates\n*.o                              # object files\n*.obj                            # Windows object files\n*.so                             # shared libraries\n*.a                              # static librarie\n*.d                              # dependency files\n*.gch                            # precompiled headers\n*.ii                             # preprocessed output\n*.ii.cpp                         # C++ preprocessed output\n*.out                            # generic executable outputs\n*.exe                            # Windows executables\n\n# 3. HIP/ROCm specific binaries and intermediates\n*.hsaco                          # ROCm compiled binary\n*.s                              # assembly output\n*.kernels.cpp                    # autogenerated kernel sources\n*.hip.cpp.*                      # hipcc intermediate outputs\n\n# 4. Official sample binaries and tutorial outputs\nbin/hipInfo                      # sample binary\nbin/hipBusBandwidth              # sample binary\nbin/hipDispatchLatency           # sample binary\nbin/hipify-clang                 # sample tool\nsamples/**/*.out                 # tutorial outputs\nsamples/**/*.code                # ISA/code dumps\nsamples/**/*.hsaco               # compiled binaries\nsamples/**/*.co                  # kernel code outputs\n\n# 5. Tags, logs and test outputs\ntags                             # ctags index\n*.log                            # log files\n/tests_output/                   # custom test output directory\n/samples_output/                 # custom sample output directory\n", #[cfg(feature = "root-iar")] Self::Iar => "# Compiled binaries\r\n*.o\r\n*.bin\r\n*.elf\r\n*.hex\r\n*.map\r\n*.out\r\n*.obj\r\n\r\n# Trash\r\n*.bak\r\nthumbs.db\r\n*.~*\r\n\r\n# IAR Settings  \r\n**/settings/*.crun  \r\n**/settings/*.dbgdt  \r\n**/settings/*.cspy  \r\n**/settings/*.cspy.*  \r\n**/settings/*.xcl  \r\n**/settings/*.dni  \r\n**/settings/*.wsdt  \r\n**/settings/*.wspos  \r\n\r\n# IAR Debug Exe  \r\n**/Exe/*.sim  \r\n\r\n# IAR Debug Obj  \r\n**/Obj/*.pbd  \r\n**/Obj/*.pbd.*  \r\n**/Obj/*.pbi  \r\n**/Obj/*.pbi.*  \r\n\r\n# IAR project \"Debug\" directory\r\nDebug/\r\n\r\n# IAR project \"Release\" directory\r\nRelease/\r\n\r\n# IAR project settings directory\r\nsettings/\r\n\r\n# IAR backup files\r\nBackup*\r\n\r\n# IAR .dep files\r\n*.dep", #[cfg(feature = "root-idris")] Self::Idris => "# Idris 2\n*.ttc\n*.ttm\n\n# Idris 1\n*.ibc\n*.o\n", #[cfg(feature = "root-igor-pro")] Self::IgorPro => "# Avoid including Experiment files: they can be created and edited locally to test the ipf files\n*.pxp\n*.pxt\n*.uxp\n*.uxt\n", #[cfg(feature = "root-j-boss")] Self::JBoss => "jboss/server/all/deploy/project.ext\njboss/server/default/deploy/project.ext\njboss/server/minimal/deploy/project.ext\njboss/server/all/log/*.log\njboss/server/all/tmp/**/*\njboss/server/all/data/**/*\njboss/server/all/work/**/*\njboss/server/default/log/*.log\njboss/server/default/tmp/**/*\njboss/server/default/data/**/*\njboss/server/default/work/**/*\njboss/server/minimal/log/*.log\njboss/server/minimal/tmp/**/*\njboss/server/minimal/data/**/*\njboss/server/minimal/work/**/*\n\n# deployed package files #\n\n*.DEPLOYED\n", #[cfg(feature = "root-java")] Self::Java => "# Compiled class file\n*.class\n\n# Log file\n*.log\n\n# BlueJ files\n*.ctxt\n\n# Mobile Tools for Java (J2ME)\n.mtj.tmp/\n\n# Package Files #\n*.jar\n*.war\n*.nar\n*.ear\n*.zip\n*.tar.gz\n*.rar\n\n# virtual machine crash logs, see http://www.java.com/en/download/help/error_hotspot.xml\nhs_err_pid*\nreplay_pid*\n", #[cfg(feature = "root-jekyll")] Self::Jekyll => "_site/\n.sass-cache/\n.jekyll-cache/\n.jekyll-metadata\n# Ignore folders generated by Bundler\n.bundle/\nvendor/\n", #[cfg(feature = "root-jenkins-home")] Self::JenkinsHome => "# Learn more about Jenkins and JENKINS_HOME directory for which this file is\n# intended.\n#\n#  http://jenkins-ci.org/\n#  https://wiki.jenkins-ci.org/display/JENKINS/Administering+Jenkins\n#\n# Note: secret.key is purposefully not tracked by git. This should be backed up\n# separately because configs may contain secrets which were encrypted using the\n# secret.key.  To back up secrets use 'tar -czf /tmp/secrets.tgz secret*' and\n# save the file separate from your repository.  If you want secrets backed up\n# with configuration, then see the bottom of this file for an example.\n\n# Ignore all JENKINS_HOME except jobs directory, root xml config, and\n# .gitignore file.\n/*\n!/jobs\n!/.gitignore\n!/*.xml\n\n# Ignore all files in jobs subdirectories except for folders.\n# Note: git doesn't track folders, only file content.\njobs/**\n!jobs/**/\n\n# Uncomment the following line to save next build numbers with config.\n\n#!jobs/**/nextBuildNumber\n\n# For performance reasons, we want to ignore builds in Jenkins jobs because it\n# contains many tiny files on large installations.  This can impact git\n# performance when running even basic commands like 'git status'.\nbuilds\nindexing\n\n# Exclude only config.xml files in repository subdirectories.\n!config.xml\n\n# Don't track workspaces (when users build on the master).\njobs/**/*workspace\n\n# Security warning: If secrets are included with your configuration, then an\n# adversary will be able to decrypt all encrypted secrets within Jenkins\n# config.  Including secrets is a bad practice, but the example is included in\n# case someone still wants it for convenience.  Uncomment the following line to\n# include secrets for decryption with repository configuration in Git.\n\n#!/secret*\n\n# As a result, only Jenkins settings and job config.xml files in JENKINS_HOME\n# will be tracked by git.\n", #[cfg(feature = "root-joomla")] Self::Joomla => "/.htaccess\n/administrator/cache/*\n/administrator/components/com_actionlogs/*\n/administrator/components/com_admin/*\n/administrator/components/com_ajax/*\n/administrator/components/com_associations/*\n/administrator/components/com_banners/*\n/administrator/components/com_cache/*\n/administrator/components/com_categories/*\n/administrator/components/com_checkin/*\n/administrator/components/com_config/*\n/administrator/components/com_contact/*\n/administrator/components/com_content/*\n/administrator/components/com_contenthistory/*\n/administrator/components/com_cpanel/*\n/administrator/components/com_fields/*\n/administrator/components/com_finder/*\n/administrator/components/com_installer/*\n/administrator/components/com_joomlaupdate/*\n/administrator/components/com_languages/*\n/administrator/components/com_login/*\n/administrator/components/com_media/*\n/administrator/components/com_menus/*\n/administrator/components/com_messages/*\n/administrator/components/com_modules/*\n/administrator/components/com_newsfeeds/*\n/administrator/components/com_plugins/*\n/administrator/components/com_postinstall/*\n/administrator/components/com_privacy/*\n/administrator/components/com_redirect/*\n/administrator/components/com_search/*\n/administrator/components/com_tags/*\n/administrator/components/com_templates/*\n/administrator/components/com_users/*\n/administrator/help/*\n/administrator/includes/*\n/administrator/index.php\n/administrator/language/en-GB/en-GB.com_actionlogs.ini\n/administrator/language/en-GB/en-GB.com_actionlogs.sys.ini\n/administrator/language/en-GB/en-GB.com_admin.ini\n/administrator/language/en-GB/en-GB.com_admin.sys.ini\n/administrator/language/en-GB/en-GB.com_ajax.ini\n/administrator/language/en-GB/en-GB.com_ajax.sys.ini\n/administrator/language/en-GB/en-GB.com_associations.ini\n/administrator/language/en-GB/en-GB.com_associations.sys.ini\n/administrator/language/en-GB/en-GB.com_banners.ini\n/administrator/language/en-GB/en-GB.com_banners.sys.ini\n/administrator/language/en-GB/en-GB.com_cache.ini\n/administrator/language/en-GB/en-GB.com_cache.sys.ini\n/administrator/language/en-GB/en-GB.com_categories.ini\n/administrator/language/en-GB/en-GB.com_categories.sys.ini\n/administrator/language/en-GB/en-GB.com_checkin.ini\n/administrator/language/en-GB/en-GB.com_checkin.sys.ini\n/administrator/language/en-GB/en-GB.com_config.ini\n/administrator/language/en-GB/en-GB.com_config.sys.ini\n/administrator/language/en-GB/en-GB.com_contact.ini\n/administrator/language/en-GB/en-GB.com_contact.sys.ini\n/administrator/language/en-GB/en-GB.com_content.ini\n/administrator/language/en-GB/en-GB.com_content.sys.ini\n/administrator/language/en-GB/en-GB.com_contenthistory.ini\n/administrator/language/en-GB/en-GB.com_contenthistory.sys.ini\n/administrator/language/en-GB/en-GB.com_cpanel.ini\n/administrator/language/en-GB/en-GB.com_cpanel.sys.ini\n/administrator/language/en-GB/en-GB.com_fields.ini\n/administrator/language/en-GB/en-GB.com_fields.sys.ini\n/administrator/language/en-GB/en-GB.com_finder.ini\n/administrator/language/en-GB/en-GB.com_finder.sys.ini\n/administrator/language/en-GB/en-GB.com_installer.ini\n/administrator/language/en-GB/en-GB.com_installer.sys.ini\n/administrator/language/en-GB/en-GB.com_joomlaupdate.ini\n/administrator/language/en-GB/en-GB.com_joomlaupdate.sys.ini\n/administrator/language/en-GB/en-GB.com_languages.ini\n/administrator/language/en-GB/en-GB.com_languages.sys.ini\n/administrator/language/en-GB/en-GB.com_login.ini\n/administrator/language/en-GB/en-GB.com_login.sys.ini\n/administrator/language/en-GB/en-GB.com_mailto.sys.ini\n/administrator/language/en-GB/en-GB.com_media.ini\n/administrator/language/en-GB/en-GB.com_media.sys.ini\n/administrator/language/en-GB/en-GB.com_menus.ini\n/administrator/language/en-GB/en-GB.com_menus.sys.ini\n/administrator/language/en-GB/en-GB.com_messages.ini\n/administrator/language/en-GB/en-GB.com_messages.sys.ini\n/administrator/language/en-GB/en-GB.com_modules.ini\n/administrator/language/en-GB/en-GB.com_modules.sys.ini\n/administrator/language/en-GB/en-GB.com_newsfeeds.ini\n/administrator/language/en-GB/en-GB.com_newsfeeds.sys.ini\n/administrator/language/en-GB/en-GB.com_plugins.ini\n/administrator/language/en-GB/en-GB.com_plugins.sys.ini\n/administrator/language/en-GB/en-GB.com_postinstall.ini\n/administrator/language/en-GB/en-GB.com_postinstall.sys.ini\n/administrator/language/en-GB/en-GB.com_privacy.ini\n/administrator/language/en-GB/en-GB.com_privacy.sys.ini\n/administrator/language/en-GB/en-GB.com_redirect.ini\n/administrator/language/en-GB/en-GB.com_redirect.sys.ini\n/administrator/language/en-GB/en-GB.com_search.ini\n/administrator/language/en-GB/en-GB.com_search.sys.ini\n/administrator/language/en-GB/en-GB.com_tags.ini\n/administrator/language/en-GB/en-GB.com_tags.sys.ini\n/administrator/language/en-GB/en-GB.com_templates.ini\n/administrator/language/en-GB/en-GB.com_templates.sys.ini\n/administrator/language/en-GB/en-GB.com_users.ini\n/administrator/language/en-GB/en-GB.com_users.sys.ini\n/administrator/language/en-GB/en-GB.com_weblinks.ini\n/administrator/language/en-GB/en-GB.com_weblinks.sys.ini\n/administrator/language/en-GB/en-GB.com_wrapper.ini\n/administrator/language/en-GB/en-GB.com_wrapper.sys.ini\n/administrator/language/en-GB/en-GB.ini\n/administrator/language/en-GB/en-GB.lib_joomla.ini\n/administrator/language/en-GB/en-GB.localise.php\n/administrator/language/en-GB/en-GB.mod_custom.ini\n/administrator/language/en-GB/en-GB.mod_custom.sys.ini\n/administrator/language/en-GB/en-GB.mod_feed.ini\n/administrator/language/en-GB/en-GB.mod_feed.sys.ini\n/administrator/language/en-GB/en-GB.mod_latest.ini\n/administrator/language/en-GB/en-GB.mod_latest.sys.ini\n/administrator/language/en-GB/en-GB.mod_latestactions.ini\n/administrator/language/en-GB/en-GB.mod_latestactions.sys.ini\n/administrator/language/en-GB/en-GB.mod_logged.ini\n/administrator/language/en-GB/en-GB.mod_logged.sys.ini\n/administrator/language/en-GB/en-GB.mod_login.ini\n/administrator/language/en-GB/en-GB.mod_login.sys.ini\n/administrator/language/en-GB/en-GB.mod_menu.ini\n/administrator/language/en-GB/en-GB.mod_menu.sys.ini\n/administrator/language/en-GB/en-GB.mod_multilangstatus.ini\n/administrator/language/en-GB/en-GB.mod_multilangstatus.sys.ini\n/administrator/language/en-GB/en-GB.mod_online.ini\n/administrator/language/en-GB/en-GB.mod_online.sys.ini\n/administrator/language/en-GB/en-GB.mod_popular.ini\n/administrator/language/en-GB/en-GB.mod_popular.sys.ini\n/administrator/language/en-GB/en-GB.mod_privacy_dashboard.ini\n/administrator/language/en-GB/en-GB.mod_privacy_dashboard.sys.ini\n/administrator/language/en-GB/en-GB.mod_quickicon.ini\n/administrator/language/en-GB/en-GB.mod_quickicon.sys.ini\n/administrator/language/en-GB/en-GB.mod_sampledata.ini\n/administrator/language/en-GB/en-GB.mod_sampledata.sys.ini\n/administrator/language/en-GB/en-GB.mod_stats_admin.ini\n/administrator/language/en-GB/en-GB.mod_stats_admin.sys.ini\n/administrator/language/en-GB/en-GB.mod_status.ini\n/administrator/language/en-GB/en-GB.mod_status.sys.ini\n/administrator/language/en-GB/en-GB.mod_submenu.ini\n/administrator/language/en-GB/en-GB.mod_submenu.sys.ini\n/administrator/language/en-GB/en-GB.mod_title.ini\n/administrator/language/en-GB/en-GB.mod_title.sys.ini\n/administrator/language/en-GB/en-GB.mod_toolbar.ini\n/administrator/language/en-GB/en-GB.mod_toolbar.sys.ini\n/administrator/language/en-GB/en-GB.mod_unread.ini\n/administrator/language/en-GB/en-GB.mod_unread.sys.ini\n/administrator/language/en-GB/en-GB.mod_version.ini\n/administrator/language/en-GB/en-GB.mod_version.sys.ini\n/administrator/language/en-GB/en-GB.plg_actionlog_joomla.ini\n/administrator/language/en-GB/en-GB.plg_actionlog_joomla.sys.ini\n/administrator/language/en-GB/en-GB.plg_authentication_cookie.ini\n/administrator/language/en-GB/en-GB.plg_authentication_cookie.sys.ini\n/administrator/language/en-GB/en-GB.plg_authentication_example.ini\n/administrator/language/en-GB/en-GB.plg_authentication_example.sys.ini\n/administrator/language/en-GB/en-GB.plg_authentication_gmail.ini\n/administrator/language/en-GB/en-GB.plg_authentication_gmail.sys.ini\n/administrator/language/en-GB/en-GB.plg_authentication_joomla.ini\n/administrator/language/en-GB/en-GB.plg_authentication_joomla.sys.ini\n/administrator/language/en-GB/en-GB.plg_authentication_ldap.ini\n/administrator/language/en-GB/en-GB.plg_authentication_ldap.sys.ini\n/administrator/language/en-GB/en-GB.plg_captcha_recaptcha.ini\n/administrator/language/en-GB/en-GB.plg_captcha_recaptcha.sys.ini\n/administrator/language/en-GB/en-GB.plg_captcha_recaptcha_invisible.ini\n/administrator/language/en-GB/en-GB.plg_captcha_recaptcha_invisible.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_confirmconsent.ini\n/administrator/language/en-GB/en-GB.plg_content_confirmconsent.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_contact.ini\n/administrator/language/en-GB/en-GB.plg_content_contact.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_emailcloak.ini\n/administrator/language/en-GB/en-GB.plg_content_emailcloak.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_fields.ini\n/administrator/language/en-GB/en-GB.plg_content_fields.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_finder.ini\n/administrator/language/en-GB/en-GB.plg_content_finder.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_geshi.ini\n/administrator/language/en-GB/en-GB.plg_content_geshi.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_joomla.ini\n/administrator/language/en-GB/en-GB.plg_content_joomla.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_loadmodule.ini\n/administrator/language/en-GB/en-GB.plg_content_loadmodule.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_pagebreak.ini\n/administrator/language/en-GB/en-GB.plg_content_pagebreak.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_pagenavigation.ini\n/administrator/language/en-GB/en-GB.plg_content_pagenavigation.sys.ini\n/administrator/language/en-GB/en-GB.plg_content_vote.ini\n/administrator/language/en-GB/en-GB.plg_content_vote.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_article.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_article.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_contact.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_contact.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_fields.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_fields.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_image.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_image.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_menu.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_menu.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_module.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_module.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_pagebreak.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_pagebreak.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_readmore.ini\n/administrator/language/en-GB/en-GB.plg_editors-xtd_readmore.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors_codemirror.ini\n/administrator/language/en-GB/en-GB.plg_editors_codemirror.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors_none.ini\n/administrator/language/en-GB/en-GB.plg_editors_none.sys.ini\n/administrator/language/en-GB/en-GB.plg_editors_tinymce.ini\n/administrator/language/en-GB/en-GB.plg_editors_tinymce.sys.ini\n/administrator/language/en-GB/en-GB.plg_extension_joomla.ini\n/administrator/language/en-GB/en-GB.plg_extension_joomla.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_calendar.ini\n/administrator/language/en-GB/en-GB.plg_fields_calendar.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_checkboxes.ini\n/administrator/language/en-GB/en-GB.plg_fields_checkboxes.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_color.ini\n/administrator/language/en-GB/en-GB.plg_fields_color.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_editor.ini\n/administrator/language/en-GB/en-GB.plg_fields_editor.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_image.ini\n/administrator/language/en-GB/en-GB.plg_fields_image.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_imagelist.ini\n/administrator/language/en-GB/en-GB.plg_fields_imagelist.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_integer.ini\n/administrator/language/en-GB/en-GB.plg_fields_integer.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_list.ini\n/administrator/language/en-GB/en-GB.plg_fields_list.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_media.ini\n/administrator/language/en-GB/en-GB.plg_fields_media.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_radio.ini\n/administrator/language/en-GB/en-GB.plg_fields_radio.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_repeatable.ini\n/administrator/language/en-GB/en-GB.plg_fields_repeatable.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_sql.ini\n/administrator/language/en-GB/en-GB.plg_fields_sql.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_text.ini\n/administrator/language/en-GB/en-GB.plg_fields_text.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_textarea.ini\n/administrator/language/en-GB/en-GB.plg_fields_textarea.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_url.ini\n/administrator/language/en-GB/en-GB.plg_fields_url.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_user.ini\n/administrator/language/en-GB/en-GB.plg_fields_user.sys.ini\n/administrator/language/en-GB/en-GB.plg_fields_usergrouplist.ini\n/administrator/language/en-GB/en-GB.plg_fields_usergrouplist.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_categories.ini\n/administrator/language/en-GB/en-GB.plg_finder_categories.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_contacts.ini\n/administrator/language/en-GB/en-GB.plg_finder_contacts.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_content.ini\n/administrator/language/en-GB/en-GB.plg_finder_content.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_newsfeeds.ini\n/administrator/language/en-GB/en-GB.plg_finder_newsfeeds.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_tags.ini\n/administrator/language/en-GB/en-GB.plg_finder_tags.sys.ini\n/administrator/language/en-GB/en-GB.plg_finder_weblinks.ini\n/administrator/language/en-GB/en-GB.plg_finder_weblinks.sys.ini\n/administrator/language/en-GB/en-GB.plg_installer_folderinstaller.ini\n/administrator/language/en-GB/en-GB.plg_installer_folderinstaller.sys.ini\n/administrator/language/en-GB/en-GB.plg_installer_packageinstaller.ini\n/administrator/language/en-GB/en-GB.plg_installer_packageinstaller.sys.ini\n/administrator/language/en-GB/en-GB.plg_installer_urlinstaller.ini\n/administrator/language/en-GB/en-GB.plg_installer_urlinstaller.sys.ini\n/administrator/language/en-GB/en-GB.plg_installer_webinstaller.ini\n/administrator/language/en-GB/en-GB.plg_installer_webinstaller.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_actionlogs.ini\n/administrator/language/en-GB/en-GB.plg_privacy_actionlogs.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_consents.ini\n/administrator/language/en-GB/en-GB.plg_privacy_consents.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_contact.ini\n/administrator/language/en-GB/en-GB.plg_privacy_contact.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_content.ini\n/administrator/language/en-GB/en-GB.plg_privacy_content.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_message.ini\n/administrator/language/en-GB/en-GB.plg_privacy_message.sys.ini\n/administrator/language/en-GB/en-GB.plg_privacy_user.ini\n/administrator/language/en-GB/en-GB.plg_privacy_user.sys.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_extensionupdate.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_extensionupdate.sys.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_joomlaupdate.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_joomlaupdate.sys.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_phpversioncheck.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_phpversioncheck.sys.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_privacycheck.ini\n/administrator/language/en-GB/en-GB.plg_quickicon_privacycheck.sys.ini\n/administrator/language/en-GB/en-GB.plg_sampledata_blog.ini\n/administrator/language/en-GB/en-GB.plg_sampledata_blog.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_categories.ini\n/administrator/language/en-GB/en-GB.plg_search_categories.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_contacts.ini\n/administrator/language/en-GB/en-GB.plg_search_contacts.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_content.ini\n/administrator/language/en-GB/en-GB.plg_search_content.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_newsfeeds.ini\n/administrator/language/en-GB/en-GB.plg_search_newsfeeds.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_tags.ini\n/administrator/language/en-GB/en-GB.plg_search_tags.sys.ini\n/administrator/language/en-GB/en-GB.plg_search_weblinks.ini\n/administrator/language/en-GB/en-GB.plg_search_weblinks.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_actionlogs.ini\n/administrator/language/en-GB/en-GB.plg_system_actionlogs.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_cache.ini\n/administrator/language/en-GB/en-GB.plg_system_cache.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_debug.ini\n/administrator/language/en-GB/en-GB.plg_system_debug.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_fields.ini\n/administrator/language/en-GB/en-GB.plg_system_fields.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_highlight.ini\n/administrator/language/en-GB/en-GB.plg_system_highlight.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_languagecode.ini\n/administrator/language/en-GB/en-GB.plg_system_languagecode.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_languagefilter.ini\n/administrator/language/en-GB/en-GB.plg_system_languagefilter.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_log.ini\n/administrator/language/en-GB/en-GB.plg_system_log.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_logout.ini\n/administrator/language/en-GB/en-GB.plg_system_logout.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_logrotation.ini\n/administrator/language/en-GB/en-GB.plg_system_logrotation.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_p3p.ini\n/administrator/language/en-GB/en-GB.plg_system_p3p.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_privacyconsent.ini\n/administrator/language/en-GB/en-GB.plg_system_privacyconsent.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_redirect.ini\n/administrator/language/en-GB/en-GB.plg_system_redirect.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_remember.ini\n/administrator/language/en-GB/en-GB.plg_system_remember.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_sef.ini\n/administrator/language/en-GB/en-GB.plg_system_sef.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_sessiongc.ini\n/administrator/language/en-GB/en-GB.plg_system_sessiongc.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_stats.ini\n/administrator/language/en-GB/en-GB.plg_system_stats.sys.ini\n/administrator/language/en-GB/en-GB.plg_system_updatenotification.ini\n/administrator/language/en-GB/en-GB.plg_system_updatenotification.sys.ini\n/administrator/language/en-GB/en-GB.plg_twofactorauth_totp.ini\n/administrator/language/en-GB/en-GB.plg_twofactorauth_totp.sys.ini\n/administrator/language/en-GB/en-GB.plg_twofactorauth_yubikey.ini\n/administrator/language/en-GB/en-GB.plg_twofactorauth_yubikey.sys.ini\n/administrator/language/en-GB/en-GB.plg_user_contactcreator.ini\n/administrator/language/en-GB/en-GB.plg_user_contactcreator.sys.ini\n/administrator/language/en-GB/en-GB.plg_user_joomla.ini\n/administrator/language/en-GB/en-GB.plg_user_joomla.sys.ini\n/administrator/language/en-GB/en-GB.plg_user_profile.ini\n/administrator/language/en-GB/en-GB.plg_user_profile.sys.ini\n/administrator/language/en-GB/en-GB.plg_user_terms.ini\n/administrator/language/en-GB/en-GB.plg_user_terms.sys.ini\n/administrator/language/en-GB/en-GB.tpl_hathor.ini\n/administrator/language/en-GB/en-GB.tpl_hathor.sys.ini\n/administrator/language/en-GB/en-GB.tpl_isis.ini\n/administrator/language/en-GB/en-GB.tpl_isis.sys.ini\n/administrator/language/en-GB/en-GB.xml\n/administrator/language/en-GB/install.xml\n/administrator/language/overrides/*\n/administrator/language/index.html\n/administrator/logs/*\n/administrator/manifests/files/joomla.xml\n/administrator/manifests/libraries/fof.xml\n/administrator/manifests/libraries/idna_convert.xml\n/administrator/manifests/libraries/joomla.xml\n/administrator/manifests/libraries/phpass.xml\n/administrator/manifests/libraries/phputf8.xml\n/administrator/manifests/packages/pkg_en-GB.xml\n/administrator/manifests/packages/index.html\n/administrator/modules/mod_custom/*\n/administrator/modules/mod_feed/*\n/administrator/modules/mod_latest/*\n/administrator/modules/mod_latestactions/*\n/administrator/modules/mod_logged/*\n/administrator/modules/mod_login/*\n/administrator/modules/mod_menu/*\n/administrator/modules/mod_multilangstatus/*\n/administrator/modules/mod_online/*\n/administrator/modules/mod_popular/*\n/administrator/modules/mod_privacy_dashboard/*\n/administrator/modules/mod_quickicon/*\n/administrator/modules/mod_sampledata/*\n/administrator/modules/mod_stats_admin/*\n/administrator/modules/mod_status/*\n/administrator/modules/mod_submenu/*\n/administrator/modules/mod_title/*\n/administrator/modules/mod_toolbar/*\n/administrator/modules/mod_unread/*\n/administrator/modules/mod_version/*\n/administrator/templates/hathor/*\n/administrator/templates/isis/*\n/administrator/templates/system/*\n/bin/*\n/cache/*\n/cli/*\n/components/com_ajax/*\n/components/com_banners/*\n/components/com_config/*\n/components/com_contact/*\n/components/com_content/*\n/components/com_contenthistory/*\n/components/com_fields/*\n/components/com_finder/*\n/components/com_mailto/*\n/components/com_media/*\n/components/com_menus/*\n/components/com_modules/*\n/components/com_newsfeeds/*\n/components/com_privacy/*\n/components/com_search/*\n/components/com_tags/*\n/components/com_users/*\n/components/com_wrapper/*\n/components/index.html\n/images/banners/*\n/images/headers/*\n/images/sampledata/*\n/images/index.html\n/images/joomla*\n/images/powered_by.png\n/includes/*\n/installation/*\n/language/en-GB/en-GB.com_ajax.ini\n/language/en-GB/en-GB.com_config.ini\n/language/en-GB/en-GB.com_contact.ini\n/language/en-GB/en-GB.com_content.ini\n/language/en-GB/en-GB.com_finder.ini\n/language/en-GB/en-GB.com_mailto.ini\n/language/en-GB/en-GB.com_media.ini\n/language/en-GB/en-GB.com_messages.ini\n/language/en-GB/en-GB.com_newsfeeds.ini\n/language/en-GB/en-GB.com_privacy.ini\n/language/en-GB/en-GB.com_search.ini\n/language/en-GB/en-GB.com_tags.ini\n/language/en-GB/en-GB.com_users.ini\n/language/en-GB/en-GB.com_weblinks.ini\n/language/en-GB/en-GB.com_wrapper.ini\n/language/en-GB/en-GB.files_joomla.sys.ini\n/language/en-GB/en-GB.finder_cli.ini\n/language/en-GB/en-GB.ini\n/language/en-GB/en-GB.lib_fof.ini\n/language/en-GB/en-GB.lib_fof.sys.ini\n/language/en-GB/en-GB.lib_idna_convert.sys.ini\n/language/en-GB/en-GB.lib_joomla.ini\n/language/en-GB/en-GB.lib_joomla.sys.ini\n/language/en-GB/en-GB.lib_phpass.sys.ini\n/language/en-GB/en-GB.lib_phpmailer.sys.ini\n/language/en-GB/en-GB.lib_phputf8.sys.ini\n/language/en-GB/en-GB.lib_simplepie.sys.ini\n/language/en-GB/en-GB.localise.php\n/language/en-GB/en-GB.mod_articles_archive.ini\n/language/en-GB/en-GB.mod_articles_archive.sys.ini\n/language/en-GB/en-GB.mod_articles_categories.ini\n/language/en-GB/en-GB.mod_articles_categories.sys.ini\n/language/en-GB/en-GB.mod_articles_category.ini\n/language/en-GB/en-GB.mod_articles_category.sys.ini\n/language/en-GB/en-GB.mod_articles_latest.ini\n/language/en-GB/en-GB.mod_articles_latest.sys.ini\n/language/en-GB/en-GB.mod_articles_news.ini\n/language/en-GB/en-GB.mod_articles_news.sys.ini\n/language/en-GB/en-GB.mod_articles_popular.ini\n/language/en-GB/en-GB.mod_articles_popular.sys.ini\n/language/en-GB/en-GB.mod_banners.ini\n/language/en-GB/en-GB.mod_banners.sys.ini\n/language/en-GB/en-GB.mod_breadcrumbs.ini\n/language/en-GB/en-GB.mod_breadcrumbs.sys.ini\n/language/en-GB/en-GB.mod_custom.ini\n/language/en-GB/en-GB.mod_custom.sys.ini\n/language/en-GB/en-GB.mod_feed.ini\n/language/en-GB/en-GB.mod_feed.sys.ini\n/language/en-GB/en-GB.mod_finder.ini\n/language/en-GB/en-GB.mod_finder.sys.ini\n/language/en-GB/en-GB.mod_footer.ini\n/language/en-GB/en-GB.mod_footer.sys.ini\n/language/en-GB/en-GB.mod_languages.ini\n/language/en-GB/en-GB.mod_languages.sys.ini\n/language/en-GB/en-GB.mod_login.ini\n/language/en-GB/en-GB.mod_login.sys.ini\n/language/en-GB/en-GB.mod_menu.ini\n/language/en-GB/en-GB.mod_menu.sys.ini\n/language/en-GB/en-GB.mod_random_image.ini\n/language/en-GB/en-GB.mod_random_image.sys.ini\n/language/en-GB/en-GB.mod_related_items.ini\n/language/en-GB/en-GB.mod_related_items.sys.ini\n/language/en-GB/en-GB.mod_search.ini\n/language/en-GB/en-GB.mod_search.sys.ini\n/language/en-GB/en-GB.mod_stats.ini\n/language/en-GB/en-GB.mod_stats.sys.ini\n/language/en-GB/en-GB.mod_syndicate.ini\n/language/en-GB/en-GB.mod_syndicate.sys.ini\n/language/en-GB/en-GB.mod_tags_popular.ini\n/language/en-GB/en-GB.mod_tags_popular.sys.ini\n/language/en-GB/en-GB.mod_tags_similar.ini\n/language/en-GB/en-GB.mod_tags_similar.sys.ini\n/language/en-GB/en-GB.mod_users_latest.ini\n/language/en-GB/en-GB.mod_users_latest.sys.ini\n/language/en-GB/en-GB.mod_weblinks.ini\n/language/en-GB/en-GB.mod_weblinks.sys.ini\n/language/en-GB/en-GB.mod_whosonline.ini\n/language/en-GB/en-GB.mod_whosonline.sys.ini\n/language/en-GB/en-GB.mod_wrapper.ini\n/language/en-GB/en-GB.mod_wrapper.sys.ini\n/language/en-GB/en-GB.tpl_atomic.ini\n/language/en-GB/en-GB.tpl_atomic.sys.ini\n/language/en-GB/en-GB.tpl_beez3.ini\n/language/en-GB/en-GB.tpl_beez3.sys.ini\n/language/en-GB/en-GB.tpl_beez5.ini\n/language/en-GB/en-GB.tpl_beez5.sys.ini\n/language/en-GB/en-GB.tpl_beez_20.ini\n/language/en-GB/en-GB.tpl_beez_20.sys.ini\n/language/en-GB/en-GB.tpl_protostar.ini\n/language/en-GB/en-GB.tpl_protostar.sys.ini\n/language/en-GB/en-GB.xml\n/language/en-GB/install.xml\n/language/overrides/*\n/language/index.html\n/layouts/joomla/*\n/layouts/libraries/*\n/layouts/plugins/*\n/layouts/index.html\n/libraries/cms/*\n/libraries/fof/*\n/libraries/idna_convert/*\n/libraries/joomla/*\n/libraries/legacy/*\n/libraries/php-encryption/*\n/libraries/phpass/*\n/libraries/phpmailer/*\n/libraries/phputf8/*\n/libraries/simplepie/*\n/libraries/src/*\n/libraries/vendor/*\n/libraries/classmap.php\n/libraries/cms.php\n/libraries/import.legacy.php\n/libraries/import.php\n/libraries/index.html\n/libraries/loader.php\n/media/cms/*\n/media/com_associations/*\n/media/com_contact/*\n/media/com_content/*\n/media/com_contenthistory/*\n/media/com_fields/*\n/media/com_finder/*\n/media/com_joomlaupdate/*\n/media/com_menus/*\n/media/com_modules/*\n/media/com_wrapper/*\n/media/contacts/*\n/media/editors/*\n/media/jui/*\n/media/mailto/*\n/media/media/*\n/media/mod_languages/*\n/media/mod_sampledata/*\n/media/overrider/*\n/media/plg_captcha_recaptcha/*\n/media/plg_captcha_recaptcha_invisible/*\n/media/plg_quickicon_extensionupdate/*\n/media/plg_quickicon_joomlaupdate/*\n/media/plg_quickicon_privacycheck/*\n/media/plg_system_highlight/*\n/media/plg_system_stats/*\n/media/plg_twofactorauth_totp/*\n/media/system/*\n/media/index.html\n/modules/mod_articles_archive/*\n/modules/mod_articles_categories/*\n/modules/mod_articles_category/*\n/modules/mod_articles_latest/*\n/modules/mod_articles_news/*\n/modules/mod_articles_popular/*\n/modules/mod_banners/*\n/modules/mod_breadcrumbs/*\n/modules/mod_custom/*\n/modules/mod_feed/*\n/modules/mod_finder/*\n/modules/mod_footer/*\n/modules/mod_languages/*\n/modules/mod_login/*\n/modules/mod_menu/*\n/modules/mod_random_image/*\n/modules/mod_related_items/*\n/modules/mod_search/*\n/modules/mod_stats/*\n/modules/mod_syndicate/*\n/modules/mod_tags_popular/*\n/modules/mod_tags_similar/*\n/modules/mod_users_latest/*\n/modules/mod_whosonline/*\n/modules/mod_wrapper/*\n/modules/index.html\n/plugins/actionlog/joomla/*\n/plugins/authentication/cookie/*\n/plugins/authentication/example/*\n/plugins/authentication/gmail/*\n/plugins/authentication/joomla/*\n/plugins/authentication/ldap/*\n/plugins/captcha/recaptcha/*\n/plugins/captcha/recaptcha_invisible/*\n/plugins/content/confirmconsent/*\n/plugins/content/contact/*\n/plugins/content/emailcloak/*\n/plugins/content/example/*\n/plugins/content/fields/*\n/plugins/content/finder/*\n/plugins/content/geshi/*\n/plugins/content/joomla/*\n/plugins/content/loadmodule/*\n/plugins/content/pagebreak/*\n/plugins/content/pagenavigation/*\n/plugins/content/vote/*\n/plugins/editors/codemirror/*\n/plugins/editors/none/*\n/plugins/editors/tinymce/*\n/plugins/editors-xtd/article/*\n/plugins/editors-xtd/contact/*\n/plugins/editors-xtd/fields/*\n/plugins/editors-xtd/image/*\n/plugins/editors-xtd/menu/*\n/plugins/editors-xtd/module/*\n/plugins/editors-xtd/pagebreak/*\n/plugins/editors-xtd/readmore/*\n/plugins/extension/example/*\n/plugins/extension/joomla/*\n/plugins/fields/calendar/*\n/plugins/fields/checkboxes/*\n/plugins/fields/color/*\n/plugins/fields/editor/*\n/plugins/fields/imagelist/*\n/plugins/fields/integer/*\n/plugins/fields/list/*\n/plugins/fields/media/*\n/plugins/fields/radio/*\n/plugins/fields/repeatable/*\n/plugins/fields/sql/*\n/plugins/fields/text/*\n/plugins/fields/textarea/*\n/plugins/fields/url/*\n/plugins/fields/user/*\n/plugins/fields/usergrouplist/*\n/plugins/finder/categories/*\n/plugins/finder/contacts/*\n/plugins/finder/content/*\n/plugins/finder/newsfeeds/*\n/plugins/finder/tags/*\n/plugins/installer/folderinstaller/*\n/plugins/installer/packageinstaller/*\n/plugins/installer/urlinstaller/*\n/plugins/privacy/actionlogs/*\n/plugins/privacy/consents/*\n/plugins/privacy/contact/*\n/plugins/privacy/content/*\n/plugins/privacy/message/*\n/plugins/privacy/user/*\n/plugins/quickicon/extensionupdate/*\n/plugins/quickicon/joomlaupdate/*\n/plugins/quickicon/phpversioncheck/*\n/plugins/quickicon/privacycheck/*\n/plugins/quickicon/index.html\n/plugins/sampledata/blog/*\n/plugins/search/categories/*\n/plugins/search/contacts/*\n/plugins/search/content/*\n/plugins/search/newsfeeds/*\n/plugins/search/tags/*\n/plugins/search/weblinks/*\n/plugins/search/index.html\n/plugins/system/actionlogs/*\n/plugins/system/cache/*\n/plugins/system/debug/*\n/plugins/system/fields/*\n/plugins/system/highlight/*\n/plugins/system/languagecode/*\n/plugins/system/languagefilter/*\n/plugins/system/log/*\n/plugins/system/logout/*\n/plugins/system/logrotation/*\n/plugins/system/p3p/*\n/plugins/system/privacyconsent/*\n/plugins/system/redirect/*\n/plugins/system/remember/*\n/plugins/system/sef/*\n/plugins/system/sessiongc/*\n/plugins/system/stats/*\n/plugins/system/updatenotification/*\n/plugins/system/index.html\n/plugins/twofactorauth/totp/*\n/plugins/twofactorauth/yubikey/*\n/plugins/user/contactcreator/*\n/plugins/user/example/*\n/plugins/user/joomla/*\n/plugins/user/profile/*\n/plugins/user/terms/*\n/plugins/user/index.html\n/plugins/index.html\n/templates/beez3/*\n/templates/protostar/*\n/templates/system/*\n/templates/index.html\n/tmp/*\n/configuration.php\n/htaccess.txt\n/index.php\n/joomla.xml\n/LICENSE.txt\n/README.txt\n/robots.txt.dist\n/web.config.txt\n", #[cfg(feature = "root-julia")] Self::Julia => "# Files generated by invoking Julia with --code-coverage\n*.jl.cov\n*.jl.*.cov\n\n# Files generated by invoking Julia with --track-allocation\n*.jl.mem\n\n# System-specific files and directories generated by the BinaryProvider and BinDeps packages\n# They contain absolute paths specific to the host computer, and so should not be committed\ndeps/deps.jl\ndeps/build.log\ndeps/downloads/\ndeps/usr/\ndeps/src/\n\n# Build artifacts for creating documentation generated by the Documenter package\ndocs/build/\ndocs/site/\n\n# File generated by Pkg, the package manager, based on a corresponding Project.toml\n# It records a fixed state of all packages used by the project. As such, it should not be\n# committed for packages, but should be committed for applications that require a static\n# environment.\nManifest*.toml\n\n# File generated by the Preferences package to store local preferences\nLocalPreferences.toml\nJuliaLocalPreferences.toml\n", #[cfg(feature = "root-katalon")] Self::Katalon => "# Katalon Test Suite\n# Compiled class file\n*.class\n*.swp\noutput\n!output/.gitkeep\nbuild\n\nLibs/TempTestCase*\nLibs/TempTestSuite*\nbin/lib/TempTestCase*\nReports/\n\\.classpath\n\\.project\n\\.settings/\nbin/lib/\nLibs/\n.svn/\n.gradle\n\n\n# Log file\n*.log\n\n# BlueJ files\n*.ctxt\n\n# Mobile Tools for Java (J2ME)\n.mtj.tmp/\n\n# Package Files #\n*.jar\n*.war\n*.ear\n*.zip\n*.tar.gz\n*.rar\n\n# virtual machine crash logs, see http://www.java.com/en/download/help/error_hotspot.xml\nhs_err_pid*\n", #[cfg(feature = "root-ki-cad")] Self::KiCad => "# For PCBs designed using KiCad: https://www.kicad.org/\n# Format documentation: https://kicad.org/help/file-formats/\n\n# Temporary files\n*.000\n*.bak\n*.bck\n*.kicad_pcb-bak\n*.kicad_sch-bak\n*-backups\n*-cache*\n*-bak\n*-bak*\n*~\n~*\n_autosave-*\n\\#auto_saved_files\\#\n*.tmp\n*-save.pro\n*-save.kicad_pcb\nfp-info-cache\n~*.lck\n\\#auto_saved_files#\n\n# Netlist files (exported from Eeschema)\n*.net\n\n# Autorouter files (exported from Pcbnew)\n*.dsn\n*.ses\n\n# Exported BOM files\n*.xml\n*.csv\n\n# Archived Backups (KiCad 6.0)\n**/*-backups/*.zip\n\n# Local project settings\n*.kicad_prl\n", #[cfg(feature = "root-kohana")] Self::Kohana => "application/cache/*\napplication/logs/*\n", #[cfg(feature = "root-kotlin")] Self::Kotlin => "# Compiled class file\n*.class\n\n# Log file\n*.log\n\n# BlueJ files\n*.ctxt\n\n# Mobile Tools for Java (J2ME)\n.mtj.tmp/\n\n# Package Files #\n*.jar\n*.war\n*.nar\n*.ear\n*.zip\n*.tar.gz\n*.rar\n\n# virtual machine crash logs, see http://www.java.com/en/download/help/error_hotspot.xml\nhs_err_pid*\nreplay_pid*\n\n# Kotlin Gradle plugin data, see https://kotlinlang.org/docs/whatsnew20.html#new-directory-for-kotlin-data-in-gradle-projects\n.kotlin/", #[cfg(feature = "root-lab-view")] Self::LabView => "# Libraries\n*.lvlibp\n*.llb\n\n# Shared objects (inc. Windows DLLs)\n*.dll\n*.so\n*.so.*\n*.dylib\n\n# Executables\n*.exe\n\n# Metadata\n*.aliases\n*.lvlps\n.cache/\n", #[cfg(feature = "root-lang-chain")] Self::LangChain => "# gitignore template for LangChain products, e.g., LangGraph, LangSmith\n# website: https://www.langchain.com/\n# website: https://www.langchain.com/langgraph\n\n# LangGraph\n.langgraph_api/\n", #[cfg(feature = "root-laravel")] Self::Laravel => "/vendor/\nnode_modules/\nnpm-debug.log\nyarn-error.log\n\n# Laravel 4 specific\nbootstrap/compiled.php\napp/storage/\n\n# Laravel 5 & Lumen specific\npublic/storage\npublic/hot\n\n# Laravel 5 & Lumen specific with changed public path\npublic_html/storage\npublic_html/hot\n\nstorage/*.key\n.env\nHomestead.yaml\nHomestead.json\n/.vagrant\n.phpunit.result.cache\n\n/public/build\n/storage/pail\n.env.backup\n.env.production\n.phpactor.json\nauth.json\n", #[cfg(feature = "root-leiningen")] Self::Leiningen => "pom.xml\npom.xml.asc\n*.jar\n*.class\n/lib/\n/classes/\n/target/\n/checkouts/\n.lein-deps-sum\n.lein-repl-history\n.lein-plugins/\n.lein-failures\n.nrepl-port\n.cpcache/\n", #[cfg(feature = "root-lemon-stand")] Self::LemonStand => "boot.php\nindex.php\ninstall.php\n/config/*\n!/config/config.php\n/controllers/*\n/init/*\n/logs/*\n/phproad/*\n/temp/*\n/uploaded/*\n/installer_files/*\n/modules/backend/*\n/modules/blog/*\n/modules/cms/*\n/modules/core/*\n/modules/session/*\n/modules/shop/*\n/modules/system/*\n/modules/users/*\n# add content_*.php if you don't want erase client changes to content\n", #[cfg(feature = "root-lilypond")] Self::Lilypond => "*.pdf\n*.ps\n*.midi\n*.mid\n*.log\n*~\n", #[cfg(feature = "root-lithium")] Self::Lithium => "libraries/*\nresources/tmp/*\n", #[cfg(feature = "root-lua")] Self::Lua => "# Compiled Lua sources\nluac.out\n\n# luarocks build files\n*.src.rock\n*.zip\n*.tar.gz\n\n# Object files\n*.o\n*.os\n*.ko\n*.obj\n*.elf\n\n# Precompiled Headers\n*.gch\n*.pch\n\n# Libraries\n*.lib\n*.a\n*.la\n*.lo\n*.def\n*.exp\n\n# Shared objects (inc. Windows DLLs)\n*.dll\n*.so\n*.so.*\n*.dylib\n\n# Executables\n*.exe\n*.out\n*.app\n*.i*86\n*.x86_64\n*.hex\n\n", #[cfg(feature = "root-luau")] Self::Luau => "# A fast, small, safe, gradually typed embeddable scripting language derived from Lua\n#\n# https://github.com/luau-lang/luau\n# https://luau.org/\n\n# Code coverage\ncoverage.out\n\n# Profiling\nprofile.out\nprofile.svg\n\n# Time trace\ntrace.json\n", #[cfg(feature = "root-magento")] Self::Magento => "#--------------------------#\n# Magento Default Files    #\n#--------------------------#\n\n/PATCH_*.sh\n\n/app/etc/local.xml\n\n/media/*\n!/media/.htaccess\n\n!/media/customer\n/media/customer/*\n!/media/customer/.htaccess\n\n!/media/dhl\n/media/dhl/*\n!/media/dhl/logo.jpg\n\n!/media/downloadable\n/media/downloadable/*\n!/media/downloadable/.htaccess\n\n!/media/xmlconnect\n/media/xmlconnect/*\n\n!/media/xmlconnect/custom\n/media/xmlconnect/custom/*\n!/media/xmlconnect/custom/ok.gif\n\n!/media/xmlconnect/original\n/media/xmlconnect/original/*\n!/media/xmlconnect/original/ok.gif\n\n!/media/xmlconnect/system\n/media/xmlconnect/system/*\n!/media/xmlconnect/system/ok.gif\n\n/var/*\n!/var/.htaccess\n\n!/var/package\n/var/package/*\n!/var/package/*.xml\n\n", #[cfg(feature = "root-maven")] Self::Maven => "target/\npom.xml.tag\npom.xml.releaseBackup\npom.xml.versionsBackup\npom.xml.next\nrelease.properties\ndependency-reduced-pom.xml\nbuildNumber.properties\n.mvn/timing.properties\n# https://maven.apache.org/wrapper/#usage-without-binary-jar\n.mvn/wrapper/maven-wrapper.jar\n\n# Eclipse m2e generated files\n# Eclipse Core\n.project\n# JDT-specific (Eclipse Java Development Tools)\n.classpath\n", #[cfg(feature = "root-mercury")] Self::Mercury => "Mercury/\nMercury.modules\n*.mh\n*.err\n*.init\n*.dll\n*.exe\n*.a\n*.so\n*.dylib\n*.beams\n*.d\n*.c_date\n", #[cfg(feature = "root-meta-programming-system")] Self::MetaProgrammingSystem => "workspace.xml\njunitvmwatcher*.properties\nbuild.properties\n\n# generated java classes and java source files\n#   manually add any custom artifacts that can't be generated from the models\n#   http://confluence.jetbrains.com/display/MPSD25/HowTo+--+MPS+and+Git\nclasses_gen\nsource_gen\nsource_gen.caches\n\n# generated test code and test results\ntest_gen\ntest_gen.caches\nTEST-*.xml\njunit*.properties\n", #[cfg(feature = "root-modelica")] Self::Modelica => "# Modelica - an object-oriented language for modeling of cyber-physical systems\n# https://modelica.org/\n# Ignore temporary files, build results, simulation files\n\n## Modelica-specific files\n*~\n*.bak\n*.bak-mo\n*.mof\n\\#*\\#\n*.moe\n*.mol\n\n## Build artefacts\n*.exe\n*.exp\n*.o\n*.pyc\n\n## Simulation files\n*.mat\n\n## Package files\n*.gz\n*.rar\n*.tar\n*.zip\n\n## Dymola-specific files\nbuildlog.txt\ndsfinal.txt\ndsin.txt\ndslog.txt\ndsmodel*\ndsres.txt\ndymosim*\nrequest\nstat\nstatus\nstop\nsuccess\n*.\n", #[cfg(feature = "root-nanoc")] Self::Nanoc => "# For projects using Nanoc (http://nanoc.ws/)\n\n# Default location for output (needs to match output_dir's value found in nanoc.yaml)\noutput/\n\n# Temporary file directory\ntmp/nanoc/\n\n# Crash Log\ncrash.log\n", #[cfg(feature = "root-nestjs")] Self::Nestjs => "# Nestjs specific\n/dist\n/node_modules\n/build\n/tmp\n\n# Logs\nlogs\n*.log\nnpm-debug.log*\npnpm-debug.log*\nyarn-debug.log*\nyarn-error.log*\nlerna-debug.log*\n\n# dotenv environment variable files\n.env\n.env.development\n.env.test\n.env.production\n\n# temp directory\n.temp\n.tmp\n", #[cfg(feature = "root-nextjs")] Self::Nextjs => "# See https://help.github.com/articles/ignoring-files/ for more about ignoring files.\n\n# dependencies\n/node_modules\n/.pnp\n.pnp.js\n\n# testing\n/coverage\n\n# next.js\n/.next/\n/out/\n\n# production\n/build\n\n# misc\n.DS_Store\n*.pem\n\n# debug\nnpm-debug.log*\nyarn-debug.log*\nyarn-error.log*\n\n# local env files\n.env*.local\n.env\n\n# vercel\n.vercel\n\n# typescript\n*.tsbuildinfo\nnext-env.d.ts\n", #[cfg(feature = "root-nim")] Self::Nim => "nimcache/\nnimblecache/\nhtmldocs/\n", #[cfg(feature = "root-nix")] Self::Nix => "# Ignore build outputs from performing a nix-build or `nix build` command\nresult\nresult-*\n\n# Ignore automatically generated direnv output\n.direnv\n", #[cfg(feature = "root-node")] Self::Node => "# Logs\nlogs\n*.log\nnpm-debug.log*\nyarn-debug.log*\nyarn-error.log*\nlerna-debug.log*\n\n# Diagnostic reports (https://nodejs.org/api/report.html)\nreport.[0-9]*.[0-9]*.[0-9]*.[0-9]*.json\n\n# Runtime data\npids\n*.pid\n*.seed\n*.pid.lock\n\n# Directory for instrumented libs generated by jscoverage/JSCover\nlib-cov\n\n# Coverage directory used by tools like istanbul\ncoverage\n*.lcov\n\n# nyc test coverage\n.nyc_output\n\n# Grunt intermediate storage (https://gruntjs.com/creating-plugins#storing-task-files)\n.grunt\n\n# Bower dependency directory (https://bower.io/)\nbower_components\n\n# node-waf configuration\n.lock-wscript\n\n# Compiled binary addons (https://nodejs.org/api/addons.html)\nbuild/Release\n\n# Dependency directories\nnode_modules/\njspm_packages/\n\n# Snowpack dependency directory (https://snowpack.dev/)\nweb_modules/\n\n# TypeScript cache\n*.tsbuildinfo\n\n# Optional npm cache directory\n.npm\n\n# Optional eslint cache\n.eslintcache\n\n# Optional stylelint cache\n.stylelintcache\n\n# Optional REPL history\n.node_repl_history\n\n# Output of 'npm pack'\n*.tgz\n\n# Yarn Integrity file\n.yarn-integrity\n\n# dotenv environment variable files\n.env\n.env.*\n!.env.example\n\n# parcel-bundler cache (https://parceljs.org/)\n.cache\n.parcel-cache\n\n# Next.js build output\n.next\nout\n\n# Nuxt.js build / generate output\n.nuxt\ndist\n\n# Gatsby files\n.cache/\n# Comment in the public line in if your project uses Gatsby and not Next.js\n# https://nextjs.org/blog/next-9-1#public-directory-support\n# public\n\n# vuepress build output\n.vuepress/dist\n\n# vuepress v2.x temp and cache directory\n.temp\n.cache\n\n# Sveltekit cache directory\n.svelte-kit/\n\n# vitepress build output\n**/.vitepress/dist\n\n# vitepress cache directory\n**/.vitepress/cache\n\n# Docusaurus cache and generated files\n.docusaurus\n\n# Serverless directories\n.serverless/\n\n# FuseBox cache\n.fusebox/\n\n# DynamoDB Local files\n.dynamodb/\n\n# Firebase cache directory\n.firebase/\n\n# TernJS port file\n.tern-port\n\n# Stores VSCode versions used for testing VSCode extensions\n.vscode-test\n\n# yarn v3\n.pnp.*\n.yarn/*\n!.yarn/patches\n!.yarn/plugins\n!.yarn/releases\n!.yarn/sdks\n!.yarn/versions\n\n# Vite logs files\nvite.config.js.timestamp-*\nvite.config.ts.timestamp-*\n", #[cfg(feature = "root-o-caml")] Self::OCaml => "*.annot\n*.cmo\n*.cma\n*.cmi\n*.a\n*.o\n*.cmx\n*.cmxs\n*.cmxa\n\n# Files containing detailed information about the compilation (generated\n# by `ocamlc`/`ocamlopt` when invoked using the option `-bin-annot`).\n# These files are typically useful for code inspection tools\n# (e.g. Merlin).\n*.cmt\n*.cmti\n\n# ocamlbuild and Dune default working directory\n_build/\n\n# ocamlbuild targets\n*.byte\n*.native\n\n# oasis generated files\nsetup.data\nsetup.log\n\n# Merlin configuring file for Vim and Emacs\n.merlin\n\n# Dune generated files\n*.install\n\n# Local OPAM switch\n_opam/\n", #[cfg(feature = "root-objective-c")] Self::ObjectiveC => "# Xcode\n#\n# gitignore contributors: remember to update Global/Xcode.gitignore, Objective-C.gitignore & Swift.gitignore\n\n## User settings\nxcuserdata/\n\n## Obj-C/Swift specific\n*.hmap\n\n## App packaging\n*.ipa\n*.dSYM.zip\n*.dSYM\n\n# CocoaPods\n#\n# We recommend against adding the Pods directory to your .gitignore. However\n# you should judge for yourself, the pros and cons are mentioned at:\n# https://guides.cocoapods.org/using/using-cocoapods.html#should-i-check-the-pods-directory-into-source-control\n#\n# Pods/\n#\n# Add this line if you want to avoid checking in source code from the Xcode workspace\n# *.xcworkspace\n\n# Carthage\n#\n# Add this line if you want to avoid checking in source code from Carthage dependencies.\n# Carthage/Checkouts\n\nCarthage/Build/\n\n# fastlane\n#\n# It is recommended to not store the screenshots in the git repo.\n# Instead, use fastlane to re-generate the screenshots whenever they are needed.\n# For more information about the recommended setup visit:\n# https://docs.fastlane.tools/best-practices/source-control/#source-control\n\nfastlane/report.xml\nfastlane/Preview.html\nfastlane/screenshots/**/*.png\nfastlane/test_output\n", #[cfg(feature = "root-opa")] Self::Opa => "_build\n_tracks\n\nopa-debug-js\n\n*.opp\n*.opx\n*.opx.broken\n*.dump\n*.api\n*.api-txt\n*.exe\n*.log\n", #[cfg(feature = "root-open-cart")] Self::OpenCart => ".htaccess\n/config.php\nadmin/config.php\n\n!index.html\n\ndownload/\nimage/data/\nimage/cache/\nsystem/cache/\nsystem/logs/\n\nsystem/storage/\n\n# vQmod log files\nvqmod/logs/*\n# vQmod cache files\nvqmod/vqcache/*\nvqmod/checked.cache\nvqmod/mods.cache\n", #[cfg(feature = "root-oracle-forms")] Self::OracleForms => "# Compiled Form Modules\n*.fmx\n\n# Compiled Menu Modules\n*.mmx\n\n# Compiled Pre-Linked Libraries\n*.plx\n", #[cfg(feature = "root-packer")] Self::Packer => "# Cache objects\npacker_cache/\n\n# Crash log\ncrash.log\n\n# https://www.packer.io/guides/hcl/variables\n# Exclude all .pkrvars.hcl files, which are likely to contain sensitive data, \n# such as password, private keys, and other secrets. These should not be part of \n# version control as they are data points which are potentially sensitive and \n# subject to change depending on the environment.\n#\n*.pkrvars.hcl\n\n# For built boxes\n*.box\n", #[cfg(feature = "root-perl")] Self::Perl => "!Build/\n.last_cover_stats\n/META.yml\n/META.json\n/MYMETA.*\n*.o\n*.pm.tdy\n*.bs\n\n# Devel::Cover\ncover_db/\n\n# Devel::NYTProf\nnytprof.out\n\n# Dist::Zilla\n/.build/\n\n# Module::Build\n_build/\nBuild\nBuild.bat\n\n# Module::Install\ninc/\n\n# ExtUtils::MakeMaker\n/blib/\n/_eumm/\n/*.gz\n/Makefile\n/Makefile.old\n/MANIFEST.bak\n/pm_to_blib\n/*.zip\n\n# Carton/Carmel\n/local/\n/.carmel/\n# cpanfile.snapshot should generally be ignored for library code, otherwise included\n# cpanfile.snapshot \n", #[cfg(feature = "root-phalcon")] Self::Phalcon => "/cache/\n/config/development/\n", #[cfg(feature = "root-play-framework")] Self::PlayFramework => "# Ignore Play! working directory #\nbin/\n/db\n.eclipse\n/lib/\n/logs/\n/modules\n/project/project\n/project/target\n/target\ntmp/\ntest-result\nserver.pid\n*.eml\n/dist/\n.cache\n", #[cfg(feature = "root-plone")] Self::Plone => "*.pyc\n*.pyo\n*.tmp*\n*.mo\n*.egg\n*.EGG\n*.egg-info\n*.EGG-INFO\n.*.cfg\nbin/\nbuild/\ndevelop-eggs/\ndownloads/\neggs/\nfake-eggs/\nparts/\ndist/\nvar/\n", #[cfg(feature = "root-prestashop")] Self::Prestashop => "# Cache, temp and personal files\n\n/.htaccess\n*.log\n\n# Cache\n/cache/*\n!/cache/.htaccess\n!/cache/cachefs/index.php\n!/cache/deprecated.txt\n!/cache/index.php\n!/cache/purifier/index.php\n!/cache/push/activity\n!/cache/push/index.php\n!/cache/push/trends\n!/cache/sandbox/index.php\n!/cache/smarty/cache/index.php\n!/cache/smarty/compile/index.php\n!/cache/smarty/index.php\n!/cache/tcpdf/index.php\n\n# Download\n/download/*\n!/download/.htaccess\n!/download/index.php\n\n# Images\n/img/*\n!/img/.htaccess\n!/img/index.php\n!/img/404.gif\n!/img/bg_500.png\n!/img/bg_loader.png\n!/img/favicon.ico\n!/img/loader.gif\n!/img/loadingAnimation.gif\n!/img/logo.jpg\n!/img/logo.png\n!/img/logo_invoice.jpg\n!/img/logo_stores.png\n!/img/macFFBgHack.png\n!/img/prestashop-avatar.png\n!/img/prestashop@2x.png\n!/img/preston-login-wink@2x.png\n!/img/preston-login@2x.png\n!/img/questionmark.png\n!/img/genders/index.php\n!/img/admin/index.php\n!/img/c/index.php\n!/img/cms/index.php\n!/img/co/index.php\n!/img/jquery-ui\n!/img/l/index.php\n!/img/m/index.php\n!/img/os/index.php\n!/img/p/index.php\n!/img/s/index.php\n!/img/scenes\n!/img/st/index.php\n!/img/su/index.php\n!/img/t/index.php\n!/img/tmp/index.php\n\n# Upload\n/upload/*\n!/upload/.htaccess\n\n/vendor/*\n/docs/phpdoc-sf/\n/composer.lock\n*.hot-update.js\n*.hot-update.json\n\n\n/admin-dev/autoupgrade/*\n!/admin-dev/autoupgrade/index.php\n!/admin-dev/autoupgrade/backup/index.php\n\n/admin-dev/backups/*\n!/admin-dev/backups/.htaccess\n\n/admin-dev/import/*\n!/admin-dev/import/.htaccess\n!/admin-dev/import/index.php\n\n/admin-dev/export/*\n!/admin-dev/export/.htaccess\n!/admin-dev/export/index.php\n\n# Downloaded RTL files\n/admin-dev/themes/default/css/bundle/default_rtl.css\n/admin-dev/themes/default/css/bundle/shared_rtl.css\n/admin-dev/themes/default/css/font_rtl.css\n/admin-dev/themes/default/css/overrides_rtl.css\n/admin-dev/themes/default/css/vendor/font-awesome/font-awesome_rtl.css\n/admin-dev/themes/default/css/vendor/nv.d3_rtl.css\n/admin-dev/themes/default/css/vendor/titatoggle-min_rtl.css\n/admin-dev/themes/default/public/theme_rtl.css\n/admin-dev/themes/new-theme/css/module/drop_rtl.css\n/admin-dev/themes/new-theme/css/right-sidebar_rtl.css\n\nthemes/*/cache/*\n\n# Config\n\nconfig/settings.inc.php\nconfig/settings.old.php\nconfig/xml/*\nconfig/themes/*\n!config/xml/themes/default.xml\nthemes/*/config/settings_*.json\napp/config/parameters.old.yml\napp/config/config.php\n\n# Themes, modules and overrides\n\nmodules/*\noverride/*\nthemes/*/\n!themes/classic\n!themes/_core\n!themes/_libraries\n\n# Vendors and dependencies\n\nbower_components/\nnode_modules/\ncomposer.phar\nphp-cs-fixer\n.grunt/*\n\n# Translations and emails templates\n\ntranslations/*\nmails/*\n!mails/themes/\n!mails/_partials/\nthemes/default-bootstrap/lang/*\nthemes/default-bootstrap/modules/*/translations/*.php\nthemes/default-bootstrap/mails/*\n!themes/default-bootstrap/mails/en/\nthemes/default-bootstrap/modules/*/mails/*\n!themes/default-bootstrap/modules/*/mails/en\n\n# MISC\n\n*sitemap.xml\n/robots.txt\n\n# Symfony\n\n/bin/\n/app/Resources/geoip/GeoLite2-City.mmdb\n/app/Resources/translations/*\n!/app/Resources/translations/default\n/app/config/parameters.yml\n/app/config/parameters.php\n/build/\n/phpunit.xml\n/var/*\n!/var/cache\n/var/cache/*\n!var/cache/.gitkeep\n!/var/logs\n/var/logs/*\n!var/logs/.gitkeep\n!/var/sessions\n/var/sessions/*\n!var/sessions/.gitkeep\n!var/SymfonyRequirements.php\n/vendor/\n/web/bundles/\n\n", #[cfg(feature = "root-processing")] Self::Processing => ".DS_Store\napplet\napplication.linux-arm64\napplication.linux-armv6hf\napplication.linux32\napplication.linux64\napplication.windows32\napplication.windows64\napplication.macosx\nout\n", #[cfg(feature = "root-pure-script")] Self::PureScript => "# Dependencies\n.psci_modules\n.spago\nbower_components\nnode_modules\n\n# Generated files\n.psci\noutput\n", #[cfg(feature = "root-python")] Self::Python => "# Byte-compiled / optimized / DLL files\n__pycache__/\n*.py[codz]\n*$py.class\n\n# C extensions\n*.so\n\n# Distribution / packaging\n.Python\nbuild/\ndevelop-eggs/\ndist/\ndownloads/\neggs/\n.eggs/\nlib/\nlib64/\nparts/\nsdist/\nvar/\nwheels/\nshare/python-wheels/\n*.egg-info/\n.installed.cfg\n*.egg\nMANIFEST\n\n# PyInstaller\n#  Usually these files are written by a python script from a template\n#  before PyInstaller builds the exe, so as to inject date/other infos into it.\n*.manifest\n*.spec\n\n# Installer logs\npip-log.txt\npip-delete-this-directory.txt\n\n# Unit test / coverage reports\nhtmlcov/\n.tox/\n.nox/\n.coverage\n.coverage.*\n.cache\nnosetests.xml\ncoverage.xml\n*.cover\n*.py.cover\n.hypothesis/\n.pytest_cache/\ncover/\n\n# Translations\n*.mo\n*.pot\n\n# Django stuff:\n*.log\nlocal_settings.py\ndb.sqlite3\ndb.sqlite3-journal\n\n# Flask stuff:\ninstance/\n.webassets-cache\n\n# Scrapy stuff:\n.scrapy\n\n# Sphinx documentation\ndocs/_build/\n\n# PyBuilder\n.pybuilder/\ntarget/\n\n# Jupyter Notebook\n.ipynb_checkpoints\n\n# IPython\nprofile_default/\nipython_config.py\n\n# pyenv\n#   For a library or package, you might want to ignore these files since the code is\n#   intended to run in multiple environments; otherwise, check them in:\n# .python-version\n\n# pipenv\n#   According to pypa/pipenv#598, it is recommended to include Pipfile.lock in version control.\n#   However, in case of collaboration, if having platform-specific dependencies or dependencies\n#   having no cross-platform support, pipenv may install dependencies that don't work, or not\n#   install all needed dependencies.\n#Pipfile.lock\n\n# UV\n#   Similar to Pipfile.lock, it is generally recommended to include uv.lock in version control.\n#   This is especially recommended for binary packages to ensure reproducibility, and is more\n#   commonly ignored for libraries.\n#uv.lock\n\n# poetry\n#   Similar to Pipfile.lock, it is generally recommended to include poetry.lock in version control.\n#   This is especially recommended for binary packages to ensure reproducibility, and is more\n#   commonly ignored for libraries.\n#   https://python-poetry.org/docs/basic-usage/#commit-your-poetrylock-file-to-version-control\n#poetry.lock\n#poetry.toml\n\n# pdm\n#   Similar to Pipfile.lock, it is generally recommended to include pdm.lock in version control.\n#   pdm recommends including project-wide configuration in pdm.toml, but excluding .pdm-python.\n#   https://pdm-project.org/en/latest/usage/project/#working-with-version-control\n#pdm.lock\n#pdm.toml\n.pdm-python\n.pdm-build/\n\n# pixi\n#   Similar to Pipfile.lock, it is generally recommended to include pixi.lock in version control.\n#pixi.lock\n#   Pixi creates a virtual environment in the .pixi directory, just like venv module creates one\n#   in the .venv directory. It is recommended not to include this directory in version control.\n.pixi\n\n# PEP 582; used by e.g. github.com/David-OConnor/pyflow and github.com/pdm-project/pdm\n__pypackages__/\n\n# Celery stuff\ncelerybeat-schedule\ncelerybeat.pid\n\n# SageMath parsed files\n*.sage.py\n\n# Environments\n.env\n.envrc\n.venv\nenv/\nvenv/\nENV/\nenv.bak/\nvenv.bak/\n\n# Spyder project settings\n.spyderproject\n.spyproject\n\n# Rope project settings\n.ropeproject\n\n# mkdocs documentation\n/site\n\n# mypy\n.mypy_cache/\n.dmypy.json\ndmypy.json\n\n# Pyre type checker\n.pyre/\n\n# pytype static type analyzer\n.pytype/\n\n# Cython debug symbols\ncython_debug/\n\n# PyCharm\n#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can\n#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore\n#  and can be added to the global gitignore or merged into this file.  For a more nuclear\n#  option (not recommended) you can uncomment the following to ignore the entire idea folder.\n#.idea/\n\n# Abstra\n# Abstra is an AI-powered process automation framework.\n# Ignore directories containing user credentials, local state, and settings.\n# Learn more at https://abstra.io/docs\n.abstra/\n\n# Visual Studio Code\n#  Visual Studio Code specific template is maintained in a separate VisualStudioCode.gitignore \n#  that can be found at https://github.com/github/gitignore/blob/main/Global/VisualStudioCode.gitignore\n#  and can be added to the global gitignore or merged into this file. However, if you prefer, \n#  you could uncomment the following to ignore the entire vscode folder\n# .vscode/\n\n# Ruff stuff:\n.ruff_cache/\n\n# PyPI configuration file\n.pypirc\n\n# Marimo\nmarimo/_static/\nmarimo/_lsp/\n__marimo__/\n\n# Streamlit\n.streamlit/secrets.toml\n", #[cfg(feature = "root-qooxdoo")] Self::Qooxdoo => "cache\ncache-downloads\ninspector\napi\nsource/inspector.html\n", #[cfg(feature = "root-qt")] Self::Qt => "# C++ objects and libs\n*.slo\n*.lo\n*.o\n*.a\n*.la\n*.lai\n*.so\n*.so.*\n*.dll\n*.dylib\n\n# Qt-es\nobject_script.*.Release\nobject_script.*.Debug\n*_plugin_import.cpp\n/.qmake.cache\n/.qmake.stash\n*.pro.user\n*.pro.user.*\n*.qbs.user\n*.qbs.user.*\n*.moc\nmoc_*.cpp\nmoc_*.h\nqrc_*.cpp\nui_*.h\n*.qmlc\n*.jsc\nMakefile*\n*build-*\n*.qm\n*.prl\n\n# Qt unit tests\ntarget_wrapper.*\n\n# QtCreator\n*.autosave\n\n# QtCreator Qml\n*.qmlproject.user\n*.qmlproject.user.*\n\n# QtCreator CMake\nCMakeLists.txt.user*\n\n# QtCreator 4.8< compilation database\ncompile_commands.json\n\n# QtCreator local machine specific files for imported projects\n*creator.user*\n\n*_qmlcache.qrc\n", #[cfg(feature = "root-r")] Self::R => "# History files\n.Rhistory\n.Rapp.history\n\n# Session Data files\n.RData\n.RDataTmp\n\n# User-specific files\n.Ruserdata\n\n# Example code in package build process\n*-Ex.R\n\n# Output files from R CMD build\n/*.tar.gz\n\n# Output files from R CMD check\n/*.Rcheck/\n\n# RStudio files\n.Rproj.user/\n\n# produced vignettes\nvignettes/*.html\nvignettes/*.pdf\n\n# OAuth2 token, see https://github.com/hadley/httr/releases/tag/v0.3\n.httr-oauth\n\n# knitr and R markdown default cache directories\n*_cache/\n/cache/\n\n# Temporary files created by R markdown\n*.utf8.md\n*.knit.md\n\n# R Environment Variables\n.Renviron\n\n# pkgdown site\ndocs/\n\n# translation temp files\npo/*~\n\n# RStudio Connect folder\nrsconnect/\n", #[cfg(feature = "root-racket")] Self::Racket => ".DS_Store\ncompiled/\n/doc/\n*~\n*.bak\n\\#*\n.\\#*\n", #[cfg(feature = "root-rails")] Self::Rails => "*.rbc\ncapybara-*.html\n.rspec\n/db/*.sqlite3\n/db/*.sqlite3-journal\n/db/*.sqlite3-[0-9]*\n/public/system\n/coverage/\n/spec/tmp\n*.orig\nrerun.txt\npickle-email-*.html\n\n# Ignore all logfiles and tempfiles.\n/log/*\n/tmp/*\n!/log/.keep\n!/tmp/.keep\n\n# TODO Comment out this rule if you are OK with secrets being uploaded to the repo\nconfig/initializers/secret_token.rb\nconfig/master.key\n\n# Only include if you have production secrets in this file, which is no longer a Rails default\n# config/secrets.yml\n\n# dotenv, dotenv-rails\n# TODO Comment out these rules if environment variables can be committed\n.env\n.env*.local\n\n## Environment normalization:\n/.bundle\n/vendor/bundle\n\n# these should all be checked in to normalize the environment:\n# Gemfile.lock, .ruby-version, .ruby-gemset\n\n# unless supporting rvm < 1.11.0 or doing something fancy, ignore this:\n.rvmrc\n\n# if using bower-rails ignore default bower_components path bower.json files\n/vendor/assets/bower_components\n*.bowerrc\nbower.json\n\n# Ignore pow environment settings\n.powenv\n\n# Ignore Byebug command history file.\n.byebug_history\n\n# Ignore node_modules\nnode_modules/\n\n# Ignore precompiled javascript packs\n/public/packs\n/public/packs-test\n/public/assets\n\n# Ignore yarn files\n/yarn-error.log\nyarn-debug.log*\n.yarn-integrity\n\n# Ignore uploaded files in development\n/storage/*\n!/storage/.keep\n/public/uploads\n", #[cfg(feature = "root-raku")] Self::Raku => "# Gitignore for Raku (https://raku.org)\n# As part of https://github.com/github/gitignore\n\n# precompiled files\n.precomp\nlib/.precomp\n\n", #[cfg(feature = "root-re-script")] Self::ReScript => "/node_modules/\n/lib/\n.bsb.lock\n", #[cfg(feature = "root-rhodes-rhomobile")] Self::RhodesRhomobile => "rholog-*\nsim-*\nbin/libs\nbin/RhoBundle\nbin/tmp\nbin/target\nbin/*.ap_\n*.o\n*.jar\n", #[cfg(feature = "root-ros")] Self::Ros => "devel/\nlogs/\nbuild/\nbin/\nlib/\nmsg_gen/\nsrv_gen/\nmsg/*Action.msg\nmsg/*ActionFeedback.msg\nmsg/*ActionGoal.msg\nmsg/*ActionResult.msg\nmsg/*Feedback.msg\nmsg/*Goal.msg\nmsg/*Result.msg\nmsg/_*.py\nbuild_isolated/\ndevel_isolated/\n\n# Generated by dynamic reconfigure\n*.cfgc\n/cfg/cpp/\n/cfg/*.py\n\n# Ignore generated docs\n*.dox\n*.wikidoc\n\n# eclipse stuff\n.project\n.cproject\n\n# qcreator stuff\nCMakeLists.txt.user\n\nsrv/_*.py\n*.pcd\n*.pyc\nqtcreator-*\n*.user\n\n/planning/cfg\n/planning/docs\n/planning/src\n\n*~\n\n# Emacs\n.#*\n\n# Catkin custom files\nCATKIN_IGNORE\n", #[cfg(feature = "root-ruby")] Self::Ruby => "*.gem\n*.rbc\n/.config\n/coverage/\n/InstalledFiles\n/pkg/\n/spec/reports/\n/spec/examples.txt\n/test/tmp/\n/test/version_tmp/\n/tmp/\n\n# Used by dotenv library to load environment variables.\n# .env\n\n# Ignore Byebug command history file.\n.byebug_history\n\n## Specific to RubyMotion:\n.dat*\n.repl_history\nbuild/\n*.bridgesupport\nbuild-iPhoneOS/\nbuild-iPhoneSimulator/\n\n## Specific to RubyMotion (use of CocoaPods):\n#\n# We recommend against adding the Pods directory to your .gitignore. However\n# you should judge for yourself, the pros and cons are mentioned at:\n# https://guides.cocoapods.org/using/using-cocoapods.html#should-i-check-the-pods-directory-into-source-control\n#\n# vendor/Pods/\n\n## Documentation cache and generated files:\n/.yardoc/\n/_yardoc/\n/doc/\n/rdoc/\n\n## Environment normalization:\n/.bundle/\n/vendor/bundle\n/lib/bundler/man/\n\n# for a library or gem, you might want to ignore these files since the code is\n# intended to run in multiple environments; otherwise, check them in:\n# Gemfile.lock\n# .ruby-version\n# .ruby-gemset\n\n# unless supporting rvm < 1.11.0 or doing something fancy, ignore this:\n.rvmrc\n\n# Used by RuboCop. Remote config files pulled in from inherit_from directive.\n# .rubocop-https?--*\n", #[cfg(feature = "root-rust")] Self::Rust => "# Generated by Cargo\n# will have compiled files and executables\ndebug\ntarget\n\n# These are backup files generated by rustfmt\n**/*.rs.bk\n\n# MSVC Windows builds of rustc generate these, which store debugging information\n*.pdb\n\n# Generated by cargo mutants\n# Contains mutation testing data\n**/mutants.out*/\n\n# RustRover\n#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can\n#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore\n#  and can be added to the global gitignore or merged into this file.  For a more nuclear\n#  option (not recommended) you can uncomment the following to ignore the entire idea folder.\n#.idea/\n", #[cfg(feature = "root-s-cons")] Self::SCons => "# for projects that use SCons for building: http://http://www.scons.org/\n.sconsign.dblite\n\n# When configure fails, SCons outputs these\nconfig.log\n.sconf_temp\n", #[cfg(feature = "root-sass")] Self::Sass => ".sass-cache/\n*.css.map\n*.sass.map\n*.scss.map\n", #[cfg(feature = "root-scala")] Self::Scala => "*.class\n*.log\n\n# virtual machine crash logs, see http://www.java.com/en/download/help/error_hotspot.xml\nhs_err_pid*\n", #[cfg(feature = "root-scheme")] Self::Scheme => "*.ss~\n*.ss#*\n.#*.ss\n\n*.scm~\n*.scm#*\n.#*.scm\n", #[cfg(feature = "root-scrivener")] Self::Scrivener => "*/Files/binder.autosave\n*/Files/binder.backup\n*/Files/search.indexes\n*/Files/user.lock\n*/Files/Docs/docs.checksum\n*/Files/Data/docs.checksum\n*/QuickLook/\n*/Settings/ui.plist\n", #[cfg(feature = "root-sdcc")] Self::Sdcc => "# SDCC stuff\n*.lnk\n*.lst\n*.map\n*.mem\n*.rel\n*.rst\n*.sym\n", #[cfg(feature = "root-seam-gen")] Self::SeamGen => "/bootstrap/data\n/bootstrap/tmp\n/classes/\n/dist/\n/exploded-archives/\n/test-build/\n/test-output/\n/test-report/\n/target/\ntemp-testng-customsuite.xml\n\n# based on http://stackoverflow.com/a/8865858/422476 I am removing inline comments\n\n#/classes/  \t\t              all class files\n#/dist/                       contains generated war files for deployment\n#/exploded-archives/\t\t      war content generation during deploy (or explode)\n#/test-build/                 test compilation (ant target for Seam)\n#/test-output/                test results\n#/test-report/                test report generation for, e.g., Hudson\n#/target/                     maven output folder\n#temp-testng-customsuite.xml\tgenerated when running test cases under Eclipse\n\n# Thanks to @VonC and @kraftan for their helpful answers on a related question\n# on StackOverflow.com:\n# http://stackoverflow.com/questions/4176687\n# /what-is-the-recommended-source-control-ignore-pattern-for-seam-projects\n", #[cfg(feature = "root-sketch-up")] Self::SketchUp => "*.skb\n", #[cfg(feature = "root-smalltalk")] Self::Smalltalk => "# changes file\n*.changes\n*.chg\n\n# system image\n*.image\n*.img7\n*.img\n\n# Pharo Smalltalk Debug log file\nPharoDebug.log\n\n# Squeak Smalltalk Debug log file\nSqueakDebug.log\n\n# Dolphin Smalltalk source file\n*.sml\n\n# Dolphin Smalltalk error file\n*.errors\n\n# Monticello package cache\n/package-cache\n\n# playground cache\n/play-cache\n/play-stash\n\n# Metacello-github cache\n/github-cache\ngithub-*.zip\n", #[cfg(feature = "root-solidity-remix")] Self::SolidityRemix => "# Remix compiler artifacts\n**/artifacts/\n**/artifacts/**\n\n# Remix plugin state folders\ndeps/\nstates/\n\n# Debug info\n*.dbg.json\n*.tsbuildinfo\n\n# Optional\n.env\n.env.local", #[cfg(feature = "root-ssdt-sqlproj")] Self::SsdtSqlproj => "## Ignore Visual Studio SSDT sqlproj specific temporary files, build results, etc\r\n## \r\n##\r\n## Get latest from https://github.com/github/gitignore/blob/master/SSDT-sqlproj.gitignore\r\n# Build output\r\nbin/\r\nobj/\r\n\r\n# DACPAC files\r\n*.dacpac\r\n\r\n# Publish profiles (optional, if environment-specific)\r\n*.publish.xml\r\n\r\n# SQL Server debug files\r\n*.dbmdl\r\n*.sqlcmdvars\r\n\r\n# Visual Studio settings\r\n.vs/\r\n\r\n# User-specific files\r\n*.user\r\n*.suo\r\n*.userosscache\r\n*.sln.docstates\r\n\r\n# Backup files\r\n*.bak\r\n*.log\r\n\r\n", #[cfg(feature = "root-stella")] Self::Stella => "# Atari 2600 (Stella) support for multiple assemblers\n# - DASM\n# - CC65\n\n# Assembled binaries and object directories\nobj/\na.out\n*.bin\n*.a26\n\n# Add in special Atari 7800-based binaries for good measure\n*.a78\n", #[cfg(feature = "root-sugar-crm")] Self::SugarCrm => "## SugarCRM\n# Ignore custom .htaccess stuff.\n/.htaccess\n# Ignore the cache directory completely.\n# This will break the current behaviour. Which was often leading to\n# the misuse of the repository as backup replacement.\n# For development the cache directory can be safely ignored and\n# therefore it is ignored.\n/cache/*\n!/cache/index.html\n# Ignore some files and directories from the custom directory.\n/custom/history/\n/custom/modulebuilder/\n/custom/working/\n/custom/modules/*/Ext/\n/custom/application/Ext/\n# Custom configuration should also be ignored.\n/config.php\n/config_override.php\n# The silent upgrade scripts aren't needed.\n/silentUpgrade*.php\n# Logs files can safely be ignored.\n*.log\n# Ignore the new upload directories.\n/upload/*\n!/upload/index.html\n/upload_backup/\n", #[cfg(feature = "root-swift")] Self::Swift => "# Xcode\n#\n# gitignore contributors: remember to update Global/Xcode.gitignore, Objective-C.gitignore & Swift.gitignore\n\n## User settings\nxcuserdata/\n\n## Obj-C/Swift specific\n*.hmap\n\n## App packaging\n*.ipa\n*.dSYM.zip\n*.dSYM\n\n## Playgrounds\ntimeline.xctimeline\nplayground.xcworkspace\n\n# Swift Package Manager\n#\n# Add this line if you want to avoid checking in source code from Swift Package Manager dependencies.\n# Packages/\n# Package.pins\n# Package.resolved\n# *.xcodeproj\n#\n# Xcode automatically generates this directory with a .xcworkspacedata file and xcuserdata\n# hence it is not needed unless you have added a package configuration file to your project\n# .swiftpm\n\n.build/\n\n# CocoaPods\n#\n# We recommend against adding the Pods directory to your .gitignore. However\n# you should judge for yourself, the pros and cons are mentioned at:\n# https://guides.cocoapods.org/using/using-cocoapods.html#should-i-check-the-pods-directory-into-source-control\n#\n# Pods/\n#\n# Add this line if you want to avoid checking in source code from the Xcode workspace\n# *.xcworkspace\n\n# Carthage\n#\n# Add this line if you want to avoid checking in source code from Carthage dependencies.\n# Carthage/Checkouts\n\nCarthage/Build/\n\n# fastlane\n#\n# It is recommended to not store the screenshots in the git repo.\n# Instead, use fastlane to re-generate the screenshots whenever they are needed.\n# For more information about the recommended setup visit:\n# https://docs.fastlane.tools/best-practices/source-control/#source-control\n\nfastlane/report.xml\nfastlane/Preview.html\nfastlane/screenshots/**/*.png\nfastlane/test_output\n", #[cfg(feature = "root-symfony")] Self::Symfony => "# Cache and logs (Symfony2)\n/app/cache/*\n/app/logs/*\n!app/cache/.gitkeep\n!app/logs/.gitkeep\n\n# Email spool folder\n/app/spool/*\n\n# Cache, session files and logs (Symfony3)\n/var/cache/*\n/var/logs/*\n/var/sessions/*\n!var/cache/.gitkeep\n!var/logs/.gitkeep\n!var/sessions/.gitkeep\n\n# Logs (Symfony4)\n/var/log/*\n!var/log/.gitkeep\n\n# Parameters\n/app/config/parameters.yml\n/app/config/parameters.ini\n\n# Managed by Composer\n/app/bootstrap.php.cache\n/var/bootstrap.php.cache\n/bin/*\n!bin/console\n!bin/symfony_requirements\n/vendor/\n\n# Assets and user uploads\n/web/bundles/\n/web/uploads/\n\n# PHPUnit\n/app/phpunit.xml\n/phpunit.xml\n\n# Build data\n/build/\n\n# Composer PHAR\n/composer.phar\n\n# Backup entities generated with doctrine:generate:entities command\n**/Entity/*~\n\n# Embedded web-server pid file\n/.web-server-pid\n", #[cfg(feature = "root-symphony-cms")] Self::SymphonyCms => "manifest/cache/\nmanifest/logs/\nmanifest/tmp/\nsymphony/\nworkspace/uploads/\ninstall-log.txt\n", #[cfg(feature = "root-te-x")] Self::TeX => "## Core latex/pdflatex auxiliary files:\n*.aux\n*.lof\n*.log\n*.lot\n*.fls\n*.out\n*.toc\n*.fmt\n*.fot\n*.cb\n*.cb2\n.*.lb\n\n## Intermediate documents:\n*.dvi\n*.xdv\n*-converted-to.*\n# these rules might exclude image files for figures etc.\n# *.ps\n# *.eps\n# *.pdf\n\n## Generated if empty string is given at \"Please type another file name for output:\"\n.pdf\n\n## Bibliography auxiliary files (bibtex/biblatex/biber):\n*.bbl\n*.bbl-SAVE-ERROR\n*.bcf\n*.bcf-SAVE-ERROR\n*.blg\n*-blx.aux\n*-blx.bib\n*.run.xml\n\n## Build tool auxiliary files:\n*.fdb_latexmk\n*.synctex\n*.synctex(busy)\n*.synctex.gz\n*.synctex.gz(busy)\n*.pdfsync\n*.rubbercache\nrubber.cache\n\n## Build tool directories for auxiliary files\n# latexrun\nlatex.out/\n\n## Auxiliary and intermediate files from other packages:\n# algorithms\n*.alg\n*.loa\n\n# achemso\nacs-*.bib\n\n# amsthm\n*.thm\n\n# attachfile2\n*.atfi\n\n# beamer\n*.nav\n*.pre\n*.snm\n*.vrb\n\n# changes\n*.soc\n*.loc\n\n# comment\n*.cut\n\n# cprotect\n*.cpt\n\n# elsarticle (documentclass of Elsevier journals)\n*.spl\n\n# endnotes\n*.ent\n\n# fixme\n*.lox\n\n# feynmf/feynmp\n*.mf\n*.mp\n*.t[1-9]\n*.t[1-9][0-9]\n*.tfm\n\n#(r)(e)ledmac/(r)(e)ledpar\n*.end\n*.?end\n*.[1-9]\n*.[1-9][0-9]\n*.[1-9][0-9][0-9]\n*.[1-9]R\n*.[1-9][0-9]R\n*.[1-9][0-9][0-9]R\n*.eledsec[1-9]\n*.eledsec[1-9]R\n*.eledsec[1-9][0-9]\n*.eledsec[1-9][0-9]R\n*.eledsec[1-9][0-9][0-9]\n*.eledsec[1-9][0-9][0-9]R\n\n# glossaries\n*.acn\n*.acr\n*.glg\n*.glg-abr\n*.glo\n*.glo-abr\n*.gls\n*.gls-abr\n*.glsdefs\n*.lzo\n*.lzs\n*.slg\n*.slo\n*.sls\n\n# uncomment this for glossaries-extra (will ignore makeindex's style files!)\n# *.ist\n\n# gnuplot\n*.gnuplot\n*.table\n\n# gnuplottex\n*-gnuplottex-*\n\n# gregoriotex\n*.gaux\n*.glog\n*.gtex\n\n# htlatex\n*.4ct\n*.4tc\n*.idv\n*.lg\n*.trc\n*.xref\n\n# hypdoc\n*.hd\n\n# hyperref\n*.brf\n\n# knitr\n*-concordance.tex\n# TODO Uncomment the next line if you use knitr and want to ignore its generated tikz files\n# *.tikz\n*-tikzDictionary\n\n# latexindent will create succesive backup files by default\n#*.bak*\n\n# listings\n*.lol\n\n# luatexja-ruby\n*.ltjruby\n\n# makeidx\n*.idx\n*.ilg\n*.ind\n\n# minitoc\n*.maf\n*.mlf\n*.mlt\n*.mtc[0-9]*\n*.slf[0-9]*\n*.slt[0-9]*\n*.stc[0-9]*\n\n# minted\n_minted*\n*.data.minted\n*.pyg\n\n# morewrites\n*.mw\n\n# newpax\n*.newpax\n\n# nomencl\n*.nlg\n*.nlo\n*.nls\n\n# pax\n*.pax\n\n# pdfpcnotes\n*.pdfpc\n\n# sagetex\n*.sagetex.sage\n*.sagetex.py\n*.sagetex.scmd\n\n# scrwfile\n*.wrt\n\n# spelling\n*.spell.bad\n*.spell.txt\n\n# svg\nsvg-inkscape/\n\n# sympy\n*.sout\n*.sympy\nsympy-plots-for-*.tex/\n\n# pdfcomment\n*.upa\n*.upb\n\n# pythontex\n*.pytxcode\npythontex-files-*/\n\n# tcolorbox\n*.listing\n\n# thmtools\n*.loe\n\n# TikZ & PGF\n*.dpth\n*.md5\n*.auxlock\n\n# titletoc\n*.ptc\n\n# todonotes\n*.tdo\n\n# vhistory\n*.hst\n*.ver\n\n# easy-todo\n*.lod\n\n# xcolor\n*.xcp\n\n# xmpincl\n*.xmpi\n\n# xindy\n*.xdy\n\n# xypic precompiled matrices and outlines\n*.xyc\n*.xyd\n\n# endfloat\n*.ttt\n*.fff\n\n# Latexian\nTSWLatexianTemp*\n\n## Editors:\n# WinEdt\n*.bak\n*.sav\n\n# latexindent.pl\n*.bak[0-9]*\n\n# Texpad\n.texpadtmp\n\n# LyX\n*.lyx~\n\n# Kile\n*.backup\n\n# gummi\n.*.swp\n\n# KBibTeX\n*~[0-9]*\n\n# TeXnicCenter\n*.tps\n\n# auto folder when using emacs and auctex\n./auto/*\n*.el\n\n# expex forward references with \\gathertags\n*-tags.tex\n\n# standalone packages\n*.sta\n\n# Makeindex log files\n*.lpz\n\n# xwatermark package\n*.xwm\n\n# REVTeX puts footnotes in the bibliography by default, unless the nofootinbib\n# option is specified. Footnotes are the stored in a file with suffix Notes.bib.\n# Uncomment the next line to have this generated file ignored.\n#*Notes.bib\n", #[cfg(feature = "root-terraform")] Self::Terraform => "# Local .terraform directories\n.terraform/\n\n# .tfstate files\n*.tfstate\n*.tfstate.*\n\n# Crash log files\ncrash.log\ncrash.*.log\n\n# Exclude all .tfvars files, which are likely to contain sensitive data, such as\n# password, private keys, and other secrets. These should not be part of version \n# control as they are data points which are potentially sensitive and subject \n# to change depending on the environment.\n*.tfvars\n*.tfvars.json\n\n# Ignore override files as they are usually used to override resources locally and so\n# are not checked in\noverride.tf\noverride.tf.json\n*_override.tf\n*_override.tf.json\n\n# Ignore transient lock info files created by terraform apply\n.terraform.tfstate.lock.info\n\n# Include override files you do wish to add to version control using negated pattern\n# !example_override.tf\n\n# Include tfplan files to ignore the plan output of command: terraform plan -out=tfplan\n# example: *tfplan*\n\n# Ignore CLI configuration files\n.terraformrc\nterraform.rc\n\n# Optional: ignore graph output files generated by `terraform graph`\n# *.dot\n\n# Optional: ignore plan files saved before destroying Terraform configuration\n# Uncomment the line below if you want to ignore planout files.\n# planout", #[cfg(feature = "root-test-complete")] Self::TestComplete => "# Test Complete ignore files: https://support.smartbear.com/viewarticle/68002/\n\n# Tester-specific Settings\n*.tcCFGExtender\n*.tcLS\n\n# Type library declarations\n*.tlb\n\n# Log files\n*.tcLogs\n\n# Backup files\n*.bak\n", #[cfg(feature = "root-textpattern")] Self::Textpattern => ".htaccess\ncss.php\nrpc/\nsites/site*/admin/\nsites/site*/private/\nsites/site*/public/admin/\nsites/site*/public/setup/\nsites/site*/public/theme/\ntextpattern/\nHISTORY.txt\nREADME.txt\n", #[cfg(feature = "root-turbo-gears2")] Self::TurboGears2 => "*.py[co]\n\n# Default development database\ndevdata.db\n\n# Default data directory\ndata/*\n\n# Packages\n*.egg\n*.egg-info\ndist\nbuild\n\n# Installer logs\npip-log.txt\n\n# Unit test / coverage reports\n.coverage\n.tox\n", #[cfg(feature = "root-twin-cat3")] Self::TwinCat3 => "### TwinCAT3 ###\n# website: https://www.beckhoff.com/twincat3/\n\n# TwinCAT PLC\n*.plcproj.bak\n*.plcproj.orig\n*.tpy\n*.tclrs\n*.library\n*.compiled-library\n*.compileinfo\n*.asm\n*.core\nLineIDs.dbg\nLineIDs.dbg.bak\n\n# TwinCAT C++ and shared types\n# ignoring the TMC file is only useful for plain PLC programming\n# as soon as shared data types (via tmc), C++ or in general TcCom-Module are used, the TMC file has to be part of the repository\n*.tmc\n*.tmcRefac\n\n# TwinCAT project files\n*.tsproj.bak\n*.tsproj.b?k\n*.tsproj.orig\n*.tspproj.bak\n*.xti.bak\n*.xti.bk?\n*.xti.orig\n*.xtv\n*.xtv.bak\n*.xtv.bk?\n*.xt?.bk?\n*.xt?.orig\n\n# Multiuser specific\n**/.TcGit/\n\n# exclude not required folders\n**/_Boot/\n**/_CompileInfo/\n**/_Libraries/\n**/_ModuleInstall/\n**/_Deployment/\n**/_Repository/\n\n\n# To include a specific library directory (i.e. third party/custom libs), \n# use pattern `!/**/_Libraries/<directory name>/` i.e. `!/**/_Libraries/www.tcunit.org/`\n# \n\n# VS Shell project specific files and folders\n**/.vs/\n*.~u\n*.project.~u\n*.suo\n", #[cfg(feature = "root-typo3")] Self::Typo3 => "## TYPO3 v6.2\n# Ignore several upload and file directories.\n/fileadmin/user_upload/\n/fileadmin/_temp_/\n/fileadmin/_processed_/\n/uploads/\n# Ignore cache\n/typo3conf/temp_CACHED*\n/typo3conf/temp_fieldInfo.php\n/typo3conf/deprecation_*.log\n/typo3conf/ENABLE_INSTALL_TOOL\n/typo3conf/realurl_autoconf.php\n/FIRST_INSTALL\n# Ignore system folders, you should have them symlinked.\n# If not comment out the following entries.\n/typo3\n/typo3_src\n/typo3_src-*\n/Packages\n/.htaccess\n/index.php\n# Ignore temp directory.\n/typo3temp/\n", #[cfg(feature = "root-unity")] Self::Unity => "# This .gitignore file should be placed at the root of your Unity project directory\n#\n# Get latest from https://github.com/github/gitignore/blob/main/Unity.gitignore\n#\n.utmp/\n/[Ll]ibrary/\n/[Tt]emp/\n/[Oo]bj/\n/[Bb]uild/\n/[Bb]uilds/\n/[Ll]ogs/\n/[Uu]ser[Ss]ettings/\n*.log\n\n# By default unity supports Blender asset imports, *.blend1 blender files do not need to be commited to version control.\n*.blend1\n*.blend1.meta\n\n# MemoryCaptures can get excessive in size.\n# They also could contain extremely sensitive data\n/[Mm]emoryCaptures/\n\n# Recordings can get excessive in size\n/[Rr]ecordings/\n\n# Uncomment this line if you wish to ignore the asset store tools plugin\n# /[Aa]ssets/AssetStoreTools*\n\n# Autogenerated Jetbrains Rider plugin\n/[Aa]ssets/Plugins/Editor/JetBrains*\n# Jetbrains Rider personal-layer settings\n*.DotSettings.user\n\n# Visual Studio cache directory\n.vs/\n\n# Gradle cache directory\n.gradle/\n\n# Autogenerated VS/MD/Consulo solution and project files\nExportedObj/\n.consulo/\n*.csproj\n*.unityproj\n*.sln\n*.suo\n*.tmp\n*.user\n*.userprefs\n*.pidb\n*.booproj\n*.svd\n*.pdb\n*.mdb\n*.opendb\n*.VC.db\n\n# Unity3D generated meta files\n*.pidb.meta\n*.pdb.meta\n*.mdb.meta\n\n# Unity3D generated file on crash reports\nsysinfo.txt\n\n# Mono auto generated files\nmono_crash.*\n\n# Builds\n*.apk\n*.aab\n*.unitypackage\n*.unitypackage.meta\n*.app\n\n# Crashlytics generated file\ncrashlytics-build.properties\n\n# TestRunner generated files\nInitTestScene*.unity*\n\n# Addressables default ignores, before user customizations\n/ServerData\n/[Aa]ssets/StreamingAssets/aa*\n/[Aa]ssets/AddressableAssetsData/link.xml*\n/[Aa]ssets/Addressables_Temp*\n# By default, Addressables content builds will generate addressables_content_state.bin\n# files in platform-specific subfolders, for example:\n# /Assets/AddressableAssetsData/OSX/addressables_content_state.bin\n/[Aa]ssets/AddressableAssetsData/*/*.bin*\n\n# Visual Scripting auto-generated files\n/[Aa]ssets/Unity.VisualScripting.Generated/VisualScripting.Flow/UnitOptions.db\n/[Aa]ssets/Unity.VisualScripting.Generated/VisualScripting.Flow/UnitOptions.db.meta\n/[Aa]ssets/Unity.VisualScripting.Generated/VisualScripting.Core/Property Providers\n/[Aa]ssets/Unity.VisualScripting.Generated/VisualScripting.Core/Property Providers.meta\n\n# Auto-generated scenes by play mode tests\n/[Aa]ssets/[Ii]nit[Tt]est[Ss]cene*.unity*\n", #[cfg(feature = "root-unreal-engine")] Self::UnrealEngine => "# Visual Studio 2015 user specific files\n.vs/\n\n# Compiled Object files\n*.slo\n*.lo\n*.o\n*.obj\n\n# Precompiled Headers\n*.gch\n*.pch\n\n# Compiled Dynamic libraries\n*.so\n*.dylib\n*.dll\n\n# Fortran module files\n*.mod\n\n# Compiled Static libraries\n*.lai\n*.la\n*.a\n*.lib\n\n# Executables\n*.exe\n*.out\n*.app\n*.ipa\n\n# These project files can be generated by the engine\n*.xcodeproj\n*.xcworkspace\n*.sln\n*.suo\n*.opensdf\n*.sdf\n*.VC.db\n*.VC.opendb\n.vsconfig\n\n# Precompiled Assets\nSourceArt/**/*.png\nSourceArt/**/*.tga\n\n# Binary Files\nBinaries/*\nPlugins/**/Binaries/*\n\n# Builds\nBuild/*\n\n# Whitelist PakBlacklist-<BuildConfiguration>.txt files\n!Build/*/\nBuild/*/**\n!Build/*/PakBlacklist*.txt\n\n# Don't ignore icon files in Build\n!Build/**/*.ico\n\n# Built data for maps\n*_BuiltData.uasset\n\n# Configuration files generated by the Editor\nSaved/*\n\n# Compiled source files for the engine to use\nIntermediate/*\nPlugins/**/Intermediate/*\n\n# Cache files for the editor to use\nDerivedDataCache/*\n", #[cfg(feature = "root-vba")] Self::Vba => "\n# Office temporary files\n~$*\n\n# Access database lock files (laccdb, ldb)\n*.[lL][aA][cC][cC][dD][bB]\n*.[lL][dD][bB]\n\n# The following sections constitute a list of Office file extensions that support VBA.\n# If you want to exclude Office files from your repo, uncomment the corresponding file extensions.\n\n# Excel (xls, xlsb, xlsm, xlt, xltm, xla, xlam)\n#*.[xX][lL][sS]\n#*.[xX][lL][sS][bB]\n#*.[xX][lL][sS][mM]\n#*.[xX][lL][tT]\n#*.[xX][lL][tT][mM]\n#*.[xX][lL][aA]\n#*.[xX][lL][aA][mM]\n\n# Word (doc, docm, dot, dotm)\n#*.[dD][oO][cC]\n#*.[dD][oO][cC][mM]\n#*.[dD][oO][tT]\n#*.[dD][oO][tT][mM]\n\n# Access (accda, accdb, accde, mdb, mde)\n#*.[aA][cC][cC][dD][aA]\n#*.[aA][cC][cC][dD][bB]\n#*.[aA][cC][cC][dD][eE]\n#*.[mM][dD][bB]\n#*.[mM][dD][eE]\n\n# PowerPoint (ppt, pptm, pot, potm, pps, ppsm)\n#*.[pP][pP][tT]\n#*.[pP][pP][tT][mM]\n#*.[pP][oO][tT]\n#*.[pP][oO][tT][mM]\n#*.[pP][pP][sS]\n#*.[pP][pP][sS][mM]\n", #[cfg(feature = "root-visual-studio")] Self::VisualStudio => "## Ignore Visual Studio temporary files, build results, and\n## files generated by popular Visual Studio add-ons.\n##\n## Get latest from https://github.com/github/gitignore/blob/main/VisualStudio.gitignore\n\n# User-specific files\n*.rsuser\n*.suo\n*.user\n*.userosscache\n*.sln.docstates\n*.env\n\n# User-specific files (MonoDevelop/Xamarin Studio)\n*.userprefs\n\n# Mono auto generated files\nmono_crash.*\n\n# Build results\n[Dd]ebug/\n[Dd]ebugPublic/\n[Rr]elease/\n[Rr]eleases/\n\n[Dd]ebug/x64/\n[Dd]ebugPublic/x64/\n[Rr]elease/x64/\n[Rr]eleases/x64/\nbin/x64/\nobj/x64/\n\n[Dd]ebug/x86/\n[Dd]ebugPublic/x86/\n[Rr]elease/x86/\n[Rr]eleases/x86/\nbin/x86/\nobj/x86/\n\n[Ww][Ii][Nn]32/\n[Aa][Rr][Mm]/\n[Aa][Rr][Mm]64/\n[Aa][Rr][Mm]64[Ee][Cc]/\nbld/\n[Oo]bj/\n[Oo]ut/\n[Ll]og/\n[Ll]ogs/\n\n# Build results on 'Bin' directories\n**/[Bb]in/*\n# Uncomment if you have tasks that rely on *.refresh files to move binaries\n# (https://github.com/github/gitignore/pull/3736)\n#!**/[Bb]in/*.refresh\n\n# Visual Studio 2015/2017 cache/options directory\n.vs/\n# Uncomment if you have tasks that create the project's static files in wwwroot\n#wwwroot/\n\n# Visual Studio 2017 auto generated files\nGenerated\\ Files/\n\n# MSTest test Results\n[Tt]est[Rr]esult*/\n[Bb]uild[Ll]og.*\n*.trx\n\n# NUnit\n*.VisualState.xml\nTestResult.xml\nnunit-*.xml\n\n# Approval Tests result files\n*.received.*\n\n# Build Results of an ATL Project\n[Dd]ebugPS/\n[Rr]eleasePS/\ndlldata.c\n\n# Benchmark Results\nBenchmarkDotNet.Artifacts/\n\n# .NET Core\nproject.lock.json\nproject.fragment.lock.json\nartifacts/\n\n# ASP.NET Scaffolding\nScaffoldingReadMe.txt\n\n# StyleCop\nStyleCopReport.xml\n\n# Files built by Visual Studio\n*_i.c\n*_p.c\n*_h.h\n*.ilk\n*.meta\n*.obj\n*.idb\n*.iobj\n*.pch\n*.pdb\n*.ipdb\n*.pgc\n*.pgd\n*.rsp\n# but not Directory.Build.rsp, as it configures directory-level build defaults\n!Directory.Build.rsp\n*.sbr\n*.tlb\n*.tli\n*.tlh\n*.tmp\n*.tmp_proj\n*_wpftmp.csproj\n*.log\n*.tlog\n*.vspscc\n*.vssscc\n.builds\n*.pidb\n*.svclog\n*.scc\n\n# Chutzpah Test files\n_Chutzpah*\n\n# Visual C++ cache files\nipch/\n*.aps\n*.ncb\n*.opendb\n*.opensdf\n*.sdf\n*.cachefile\n*.VC.db\n*.VC.VC.opendb\n\n# Visual Studio profiler\n*.psess\n*.vsp\n*.vspx\n*.sap\n\n# Visual Studio Trace Files\n*.e2e\n\n# TFS 2012 Local Workspace\n$tf/\n\n# Guidance Automation Toolkit\n*.gpState\n\n# ReSharper is a .NET coding add-in\n_ReSharper*/\n*.[Rr]e[Ss]harper\n*.DotSettings.user\n\n# TeamCity is a build add-in\n_TeamCity*\n\n# DotCover is a Code Coverage Tool\n*.dotCover\n\n# AxoCover is a Code Coverage Tool\n.axoCover/*\n!.axoCover/settings.json\n\n# Coverlet is a free, cross platform Code Coverage Tool\ncoverage*.json\ncoverage*.xml\ncoverage*.info\n\n# Visual Studio code coverage results\n*.coverage\n*.coveragexml\n\n# NCrunch\n_NCrunch_*\n.NCrunch_*\n.*crunch*.local.xml\nnCrunchTemp_*\n\n# MightyMoose\n*.mm.*\nAutoTest.Net/\n\n# Web workbench (sass)\n.sass-cache/\n\n# Installshield output folder\n[Ee]xpress/\n\n# DocProject is a documentation generator add-in\nDocProject/buildhelp/\nDocProject/Help/*.HxT\nDocProject/Help/*.HxC\nDocProject/Help/*.hhc\nDocProject/Help/*.hhk\nDocProject/Help/*.hhp\nDocProject/Help/Html2\nDocProject/Help/html\n\n# Click-Once directory\npublish/\n\n# Publish Web Output\n*.[Pp]ublish.xml\n*.azurePubxml\n# Note: Comment the next line if you want to checkin your web deploy settings,\n# but database connection strings (with potential passwords) will be unencrypted\n*.pubxml\n*.publishproj\n\n# Microsoft Azure Web App publish settings. Comment the next line if you want to\n# checkin your Azure Web App publish settings, but sensitive information contained\n# in these scripts will be unencrypted\nPublishScripts/\n\n# NuGet Packages\n*.nupkg\n# NuGet Symbol Packages\n*.snupkg\n# The packages folder can be ignored because of Package Restore\n**/[Pp]ackages/*\n# except build/, which is used as an MSBuild target.\n!**/[Pp]ackages/build/\n# Uncomment if necessary however generally it will be regenerated when needed\n#!**/[Pp]ackages/repositories.config\n# NuGet v3's project.json files produces more ignorable files\n*.nuget.props\n*.nuget.targets\n\n# Microsoft Azure Build Output\ncsx/\n*.build.csdef\n\n# Microsoft Azure Emulator\necf/\nrcf/\n\n# Windows Store app package directories and files\nAppPackages/\nBundleArtifacts/\nPackage.StoreAssociation.xml\n_pkginfo.txt\n*.appx\n*.appxbundle\n*.appxupload\n\n# Visual Studio cache files\n# files ending in .cache can be ignored\n*.[Cc]ache\n# but keep track of directories ending in .cache\n!?*.[Cc]ache/\n\n# Others\nClientBin/\n~$*\n*~\n*.dbmdl\n*.dbproj.schemaview\n*.jfm\n*.pfx\n*.publishsettings\norleans.codegen.cs\n\n# Including strong name files can present a security risk\n# (https://github.com/github/gitignore/pull/2483#issue-259490424)\n#*.snk\n\n# Since there are multiple workflows, uncomment next line to ignore bower_components\n# (https://github.com/github/gitignore/pull/1529#issuecomment-104372622)\n#bower_components/\n\n# RIA/Silverlight projects\nGenerated_Code/\n\n# Backup & report files from converting an old project file\n# to a newer Visual Studio version. Backup files are not needed,\n# because we have git ;-)\n_UpgradeReport_Files/\nBackup*/\nUpgradeLog*.XML\nUpgradeLog*.htm\nServiceFabricBackup/\n*.rptproj.bak\n\n# SQL Server files\n*.mdf\n*.ldf\n*.ndf\n\n# Business Intelligence projects\n*.rdl.data\n*.bim.layout\n*.bim_*.settings\n*.rptproj.rsuser\n*- [Bb]ackup.rdl\n*- [Bb]ackup ([0-9]).rdl\n*- [Bb]ackup ([0-9][0-9]).rdl\n\n# Microsoft Fakes\nFakesAssemblies/\n\n# GhostDoc plugin setting file\n*.GhostDoc.xml\n\n# Node.js Tools for Visual Studio\n.ntvs_analysis.dat\nnode_modules/\n\n# Visual Studio 6 build log\n*.plg\n\n# Visual Studio 6 workspace options file\n*.opt\n\n# Visual Studio 6 auto-generated workspace file (contains which files were open etc.)\n*.vbw\n\n# Visual Studio 6 workspace and project file (working project files containing files to include in project)\n*.dsw\n*.dsp\n\n# Visual Studio 6 technical files\n*.ncb\n*.aps\n\n# Visual Studio LightSwitch build output\n**/*.HTMLClient/GeneratedArtifacts\n**/*.DesktopClient/GeneratedArtifacts\n**/*.DesktopClient/ModelManifest.xml\n**/*.Server/GeneratedArtifacts\n**/*.Server/ModelManifest.xml\n_Pvt_Extensions\n\n# Paket dependency manager\n**/.paket/paket.exe\npaket-files/\n\n# FAKE - F# Make\n**/.fake/\n\n# CodeRush personal settings\n**/.cr/personal\n\n# Python Tools for Visual Studio (PTVS)\n**/__pycache__/\n*.pyc\n\n# Cake - Uncomment if you are using it\n#tools/**\n#!tools/packages.config\n\n# Tabs Studio\n*.tss\n\n# Telerik's JustMock configuration file\n*.jmconfig\n\n# BizTalk build output\n*.btp.cs\n*.btm.cs\n*.odx.cs\n*.xsd.cs\n\n# OpenCover UI analysis results\nOpenCover/\n\n# Azure Stream Analytics local run output\nASALocalRun/\n\n# MSBuild Binary and Structured Log\n*.binlog\nMSBuild_Logs/\n\n# AWS SAM Build and Temporary Artifacts folder\n.aws-sam\n\n# NVidia Nsight GPU debugger configuration file\n*.nvuser\n\n# MFractors (Xamarin productivity tool) working folder\n**/.mfractor/\n\n# Local History for Visual Studio\n**/.localhistory/\n\n# Visual Studio History (VSHistory) files\n.vshistory/\n\n# BeatPulse healthcheck temp database\nhealthchecksdb\n\n# Backup folder for Package Reference Convert tool in Visual Studio 2017\nMigrationBackup/\n\n# Ionide (cross platform F# VS Code tools) working folder\n**/.ionide/\n\n# Fody - auto-generated XML schema\nFodyWeavers.xsd\n\n# VS Code files for those working on multiple tools\n.vscode/*\n!.vscode/settings.json\n!.vscode/tasks.json\n!.vscode/launch.json\n!.vscode/extensions.json\n!.vscode/*.code-snippets\n\n# Local History for Visual Studio Code\n.history/\n\n# Built Visual Studio Code Extensions\n*.vsix\n\n# Windows Installer files from build outputs\n*.cab\n*.msi\n*.msix\n*.msm\n*.msp\n", #[cfg(feature = "root-vvvv")] Self::Vvvv => "\n# .v4p backup files\n*~.xml\n\n# Dynamic plugins .dll\nbin/\n", #[cfg(feature = "root-waf")] Self::Waf => "# For projects that use the Waf build system: https://waf.io/\n# Dot-hidden on Unix-like systems\n.waf-*-*/\n.waf3-*-*/\n# Hidden directory on Windows (no dot)\nwaf-*-*/\nwaf3-*-*/\n# Lockfile\n.lock-waf_*_build\n", #[cfg(feature = "root-word-press")] Self::WordPress => "# Wordpress - ignore core, configuration, examples, uploads and logs.\n# https://github.com/github/gitignore/blob/main/WordPress.gitignore\n\n# Core\n#\n# Note: if you want to stage/commit WP core files\n# you can delete this whole section/until Configuration.\n/wp-admin/\n/wp-content/index.php\n/wp-content/languages\n/wp-content/plugins/index.php\n/wp-content/themes/index.php\n/wp-includes/\n/index.php\n/license.txt\n/readme.html\n/wp-*.php\n/xmlrpc.php\n\n# Configuration\nwp-config.php\n\n# Example themes\n/wp-content/themes/twenty*/\n\n# Example plugin\n/wp-content/plugins/hello.php\n\n# Uploads\n/wp-content/uploads/\n\n# Log files\n*.log\n\n# htaccess\n/.htaccess\n\n# All plugins\n#\n# Note: If you wish to whitelist plugins,\n# uncomment the next line\n#/wp-content/plugins\n\n# All themes\n#\n# Note: If you wish to whitelist themes,\n# uncomment the next line\n#/wp-content/themes", #[cfg(feature = "root-xojo")] Self::Xojo => "# Xojo (formerly REALbasic and Real Studio)\n\nBuilds*\n*.debug\n*.debug.app\nDebug*.exe\nDebug*/Debug*.exe\nDebug*/Debug*\\ Libs\n*.rbuistate\n*.xojo_uistate\n*.obsolete*\n", #[cfg(feature = "root-yeoman")] Self::Yeoman => "node_modules/\nbower_components/\n*.log\n\nbuild/\ndist/\n", #[cfg(feature = "root-yii")] Self::Yii => "assets/*\n!assets/.gitignore\nprotected/runtime/*\n!protected/runtime/.gitignore\nprotected/data/*.db\nthemes/classic/views/\n", #[cfg(feature = "root-zend-framework")] Self::ZendFramework => "# Composer files\ncomposer.phar\nvendor/\n\n# Local configs\nconfig/autoload/*.local.php\n\n# Binary gettext files\n*.mo\n\n# Data\ndata/logs/\ndata/cache/\ndata/sessions/\ndata/tmp/\ntemp/\n\n#Doctrine 2\ndata/DoctrineORMModule/Proxy/\ndata/DoctrineORMModule/cache/\n\n# Legacy ZF1\ndemos/\nextras/documentation\n", #[cfg(feature = "root-zephir")] Self::Zephir => "# Cache files, generates by Zephir\n.temp/\n.libs/\n\n# Object files, generates by linker\n*.lo\n*.la\n*.o\n*.loT\n\n# Files generated by configure and Zephir,\n# not required for extension compilation.\next/build/\next/modules/\next/Makefile*\next/config*\next/acinclude.m4\next/aclocal.m4\next/autom4te*\next/install-sh\next/ltmain.sh\next/missing\next/mkinstalldirs\next/run-tests.php\next/.deps\next/libtool\n", #[cfg(feature = "root-zig")] Self::Zig => ".zig-cache/\nzig-out/\n*.o\n" }
	}

	fn file_name(self) -> &'static str {
		match self {
			#[cfg(feature = "root-actionscript")]
			Self::Actionscript => "Actionscript.gitignore",
			#[cfg(feature = "root-ada")]
			Self::Ada => "Ada.gitignore",
			#[cfg(feature = "root-adventure-game-studio")]
			Self::AdventureGameStudio => "AdventureGameStudio.gitignore",
			#[cfg(feature = "root-agda")]
			Self::Agda => "Agda.gitignore",
			#[cfg(feature = "root-al")]
			Self::Al => "AL.gitignore",
			#[cfg(feature = "root-android")]
			Self::Android => "Android.gitignore",
			#[cfg(feature = "root-angular")]
			Self::Angular => "Angular.gitignore",
			#[cfg(feature = "root-app-engine")]
			Self::AppEngine => "AppEngine.gitignore",
			#[cfg(feature = "root-appcelerator-titanium")]
			Self::AppceleratorTitanium => "AppceleratorTitanium.gitignore",
			#[cfg(feature = "root-arch-linux-packages")]
			Self::ArchLinuxPackages => "ArchLinuxPackages.gitignore",
			#[cfg(feature = "root-autotools")]
			Self::Autotools => "Autotools.gitignore",
			#[cfg(feature = "root-ballerina")]
			Self::Ballerina => "Ballerina.gitignore",
			#[cfg(feature = "root-c")]
			Self::C => "C.gitignore",
			#[cfg(feature = "root-c-make")]
			Self::CMake => "CMake.gitignore",
			#[cfg(feature = "root-c-plus-plus")]
			Self::CPlusPlus => "C++.gitignore",
			#[cfg(feature = "root-cake-php")]
			Self::CakePhp => "CakePHP.gitignore",
			#[cfg(feature = "root-cf-wheels")]
			Self::CfWheels => "CFWheels.gitignore",
			#[cfg(feature = "root-chef-cookbook")]
			Self::ChefCookbook => "ChefCookbook.gitignore",
			#[cfg(feature = "root-clojure")]
			Self::Clojure => "Clojure.gitignore",
			#[cfg(feature = "root-code-igniter")]
			Self::CodeIgniter => "CodeIgniter.gitignore",
			#[cfg(feature = "root-common-lisp")]
			Self::CommonLisp => "CommonLisp.gitignore",
			#[cfg(feature = "root-composer")]
			Self::Composer => "Composer.gitignore",
			#[cfg(feature = "root-concrete5")]
			Self::Concrete5 => "Concrete5.gitignore",
			#[cfg(feature = "root-coq")]
			Self::Coq => "Coq.gitignore",
			#[cfg(feature = "root-craft-cms")]
			Self::CraftCms => "CraftCMS.gitignore",
			#[cfg(feature = "root-cuda")]
			Self::Cuda => "CUDA.gitignore",
			#[cfg(feature = "root-d")]
			Self::D => "D.gitignore",
			#[cfg(feature = "root-dart")]
			Self::Dart => "Dart.gitignore",
			#[cfg(feature = "root-delphi")]
			Self::Delphi => "Delphi.gitignore",
			#[cfg(feature = "root-dm")]
			Self::Dm => "DM.gitignore",
			#[cfg(feature = "root-dotnet")]
			Self::Dotnet => "Dotnet.gitignore",
			#[cfg(feature = "root-drupal")]
			Self::Drupal => "Drupal.gitignore",
			#[cfg(feature = "root-e-pi-server")]
			Self::EPiServer => "EPiServer.gitignore",
			#[cfg(feature = "root-eagle")]
			Self::Eagle => "Eagle.gitignore",
			#[cfg(feature = "root-ecutest")]
			Self::Ecutest => "ecu.test.gitignore",
			#[cfg(feature = "root-elisp")]
			Self::Elisp => "Elisp.gitignore",
			#[cfg(feature = "root-elixir")]
			Self::Elixir => "Elixir.gitignore",
			#[cfg(feature = "root-elm")]
			Self::Elm => "Elm.gitignore",
			#[cfg(feature = "root-erlang")]
			Self::Erlang => "Erlang.gitignore",
			#[cfg(feature = "root-expression-engine")]
			Self::ExpressionEngine => "ExpressionEngine.gitignore",
			#[cfg(feature = "root-ext-js")]
			Self::ExtJs => "ExtJs.gitignore",
			#[cfg(feature = "root-fancy")]
			Self::Fancy => "Fancy.gitignore",
			#[cfg(feature = "root-finale")]
			Self::Finale => "Finale.gitignore",
			#[cfg(feature = "root-firebase")]
			Self::Firebase => "Firebase.gitignore",
			#[cfg(feature = "root-flax-engine")]
			Self::FlaxEngine => "FlaxEngine.gitignore",
			#[cfg(feature = "root-flutter")]
			Self::Flutter => "Flutter.gitignore",
			#[cfg(feature = "root-force-dot-com")]
			Self::ForceDotCom => "ForceDotCom.gitignore",
			#[cfg(feature = "root-fortran")]
			Self::Fortran => "Fortran.gitignore",
			#[cfg(feature = "root-fuel-php")]
			Self::FuelPhp => "FuelPHP.gitignore",
			#[cfg(feature = "root-gcov")]
			Self::Gcov => "Gcov.gitignore",
			#[cfg(feature = "root-git-book")]
			Self::GitBook => "GitBook.gitignore",
			#[cfg(feature = "root-git-hub-pages")]
			Self::GitHubPages => "GitHubPages.gitignore",
			#[cfg(feature = "root-gleam")]
			Self::Gleam => "Gleam.gitignore",
			#[cfg(feature = "root-go")]
			Self::Go => "Go.gitignore",
			#[cfg(feature = "root-godot")]
			Self::Godot => "Godot.gitignore",
			#[cfg(feature = "root-gradle")]
			Self::Gradle => "Gradle.gitignore",
			#[cfg(feature = "root-grails")]
			Self::Grails => "Grails.gitignore",
			#[cfg(feature = "root-gwt")]
			Self::Gwt => "GWT.gitignore",
			#[cfg(feature = "root-haskell")]
			Self::Haskell => "Haskell.gitignore",
			#[cfg(feature = "root-haxe")]
			Self::Haxe => "Haxe.gitignore",
			#[cfg(feature = "root-hip")]
			Self::Hip => "HIP.gitignore",
			#[cfg(feature = "root-iar")]
			Self::Iar => "IAR.gitignore",
			#[cfg(feature = "root-idris")]
			Self::Idris => "Idris.gitignore",
			#[cfg(feature = "root-igor-pro")]
			Self::IgorPro => "IGORPro.gitignore",
			#[cfg(feature = "root-j-boss")]
			Self::JBoss => "JBoss.gitignore",
			#[cfg(feature = "root-java")]
			Self::Java => "Java.gitignore",
			#[cfg(feature = "root-jekyll")]
			Self::Jekyll => "Jekyll.gitignore",
			#[cfg(feature = "root-jenkins-home")]
			Self::JenkinsHome => "JENKINS_HOME.gitignore",
			#[cfg(feature = "root-joomla")]
			Self::Joomla => "Joomla.gitignore",
			#[cfg(feature = "root-julia")]
			Self::Julia => "Julia.gitignore",
			#[cfg(feature = "root-katalon")]
			Self::Katalon => "Katalon.gitignore",
			#[cfg(feature = "root-ki-cad")]
			Self::KiCad => "KiCad.gitignore",
			#[cfg(feature = "root-kohana")]
			Self::Kohana => "Kohana.gitignore",
			#[cfg(feature = "root-kotlin")]
			Self::Kotlin => "Kotlin.gitignore",
			#[cfg(feature = "root-lab-view")]
			Self::LabView => "LabVIEW.gitignore",
			#[cfg(feature = "root-lang-chain")]
			Self::LangChain => "LangChain.gitignore",
			#[cfg(feature = "root-laravel")]
			Self::Laravel => "Laravel.gitignore",
			#[cfg(feature = "root-leiningen")]
			Self::Leiningen => "Leiningen.gitignore",
			#[cfg(feature = "root-lemon-stand")]
			Self::LemonStand => "LemonStand.gitignore",
			#[cfg(feature = "root-lilypond")]
			Self::Lilypond => "Lilypond.gitignore",
			#[cfg(feature = "root-lithium")]
			Self::Lithium => "Lithium.gitignore",
			#[cfg(feature = "root-lua")]
			Self::Lua => "Lua.gitignore",
			#[cfg(feature = "root-luau")]
			Self::Luau => "Luau.gitignore",
			#[cfg(feature = "root-magento")]
			Self::Magento => "Magento.gitignore",
			#[cfg(feature = "root-maven")]
			Self::Maven => "Maven.gitignore",
			#[cfg(feature = "root-mercury")]
			Self::Mercury => "Mercury.gitignore",
			#[cfg(feature = "root-meta-programming-system")]
			Self::MetaProgrammingSystem => "MetaProgrammingSystem.gitignore",
			#[cfg(feature = "root-modelica")]
			Self::Modelica => "Modelica.gitignore",
			#[cfg(feature = "root-nanoc")]
			Self::Nanoc => "Nanoc.gitignore",
			#[cfg(feature = "root-nestjs")]
			Self::Nestjs => "Nestjs.gitignore",
			#[cfg(feature = "root-nextjs")]
			Self::Nextjs => "Nextjs.gitignore",
			#[cfg(feature = "root-nim")]
			Self::Nim => "Nim.gitignore",
			#[cfg(feature = "root-nix")]
			Self::Nix => "Nix.gitignore",
			#[cfg(feature = "root-node")]
			Self::Node => "Node.gitignore",
			#[cfg(feature = "root-o-caml")]
			Self::OCaml => "OCaml.gitignore",
			#[cfg(feature = "root-objective-c")]
			Self::ObjectiveC => "Objective-C.gitignore",
			#[cfg(feature = "root-opa")]
			Self::Opa => "Opa.gitignore",
			#[cfg(feature = "root-open-cart")]
			Self::OpenCart => "OpenCart.gitignore",
			#[cfg(feature = "root-oracle-forms")]
			Self::OracleForms => "OracleForms.gitignore",
			#[cfg(feature = "root-packer")]
			Self::Packer => "Packer.gitignore",
			#[cfg(feature = "root-perl")]
			Self::Perl => "Perl.gitignore",
			#[cfg(feature = "root-phalcon")]
			Self::Phalcon => "Phalcon.gitignore",
			#[cfg(feature = "root-play-framework")]
			Self::PlayFramework => "PlayFramework.gitignore",
			#[cfg(feature = "root-plone")]
			Self::Plone => "Plone.gitignore",
			#[cfg(feature = "root-prestashop")]
			Self::Prestashop => "Prestashop.gitignore",
			#[cfg(feature = "root-processing")]
			Self::Processing => "Processing.gitignore",
			#[cfg(feature = "root-pure-script")]
			Self::PureScript => "PureScript.gitignore",
			#[cfg(feature = "root-python")]
			Self::Python => "Python.gitignore",
			#[cfg(feature = "root-qooxdoo")]
			Self::Qooxdoo => "Qooxdoo.gitignore",
			#[cfg(feature = "root-qt")]
			Self::Qt => "Qt.gitignore",
			#[cfg(feature = "root-r")]
			Self::R => "R.gitignore",
			#[cfg(feature = "root-racket")]
			Self::Racket => "Racket.gitignore",
			#[cfg(feature = "root-rails")]
			Self::Rails => "Rails.gitignore",
			#[cfg(feature = "root-raku")]
			Self::Raku => "Raku.gitignore",
			#[cfg(feature = "root-re-script")]
			Self::ReScript => "ReScript.gitignore",
			#[cfg(feature = "root-rhodes-rhomobile")]
			Self::RhodesRhomobile => "RhodesRhomobile.gitignore",
			#[cfg(feature = "root-ros")]
			Self::Ros => "ROS.gitignore",
			#[cfg(feature = "root-ruby")]
			Self::Ruby => "Ruby.gitignore",
			#[cfg(feature = "root-rust")]
			Self::Rust => "Rust.gitignore",
			#[cfg(feature = "root-s-cons")]
			Self::SCons => "SCons.gitignore",
			#[cfg(feature = "root-sass")]
			Self::Sass => "Sass.gitignore",
			#[cfg(feature = "root-scala")]
			Self::Scala => "Scala.gitignore",
			#[cfg(feature = "root-scheme")]
			Self::Scheme => "Scheme.gitignore",
			#[cfg(feature = "root-scrivener")]
			Self::Scrivener => "Scrivener.gitignore",
			#[cfg(feature = "root-sdcc")]
			Self::Sdcc => "Sdcc.gitignore",
			#[cfg(feature = "root-seam-gen")]
			Self::SeamGen => "SeamGen.gitignore",
			#[cfg(feature = "root-sketch-up")]
			Self::SketchUp => "SketchUp.gitignore",
			#[cfg(feature = "root-smalltalk")]
			Self::Smalltalk => "Smalltalk.gitignore",
			#[cfg(feature = "root-solidity-remix")]
			Self::SolidityRemix => "Solidity-Remix.gitignore",
			#[cfg(feature = "root-ssdt-sqlproj")]
			Self::SsdtSqlproj => "SSDT-sqlproj.gitignore",
			#[cfg(feature = "root-stella")]
			Self::Stella => "Stella.gitignore",
			#[cfg(feature = "root-sugar-crm")]
			Self::SugarCrm => "SugarCRM.gitignore",
			#[cfg(feature = "root-swift")]
			Self::Swift => "Swift.gitignore",
			#[cfg(feature = "root-symfony")]
			Self::Symfony => "Symfony.gitignore",
			#[cfg(feature = "root-symphony-cms")]
			Self::SymphonyCms => "SymphonyCMS.gitignore",
			#[cfg(feature = "root-te-x")]
			Self::TeX => "TeX.gitignore",
			#[cfg(feature = "root-terraform")]
			Self::Terraform => "Terraform.gitignore",
			#[cfg(feature = "root-test-complete")]
			Self::TestComplete => "TestComplete.gitignore",
			#[cfg(feature = "root-textpattern")]
			Self::Textpattern => "Textpattern.gitignore",
			#[cfg(feature = "root-turbo-gears2")]
			Self::TurboGears2 => "TurboGears2.gitignore",
			#[cfg(feature = "root-twin-cat3")]
			Self::TwinCat3 => "TwinCAT3.gitignore",
			#[cfg(feature = "root-typo3")]
			Self::Typo3 => "Typo3.gitignore",
			#[cfg(feature = "root-unity")]
			Self::Unity => "Unity.gitignore",
			#[cfg(feature = "root-unreal-engine")]
			Self::UnrealEngine => "UnrealEngine.gitignore",
			#[cfg(feature = "root-vba")]
			Self::Vba => "VBA.gitignore",
			#[cfg(feature = "root-visual-studio")]
			Self::VisualStudio => "VisualStudio.gitignore",
			#[cfg(feature = "root-vvvv")]
			Self::Vvvv => "VVVV.gitignore",
			#[cfg(feature = "root-waf")]
			Self::Waf => "Waf.gitignore",
			#[cfg(feature = "root-word-press")]
			Self::WordPress => "WordPress.gitignore",
			#[cfg(feature = "root-xojo")]
			Self::Xojo => "Xojo.gitignore",
			#[cfg(feature = "root-yeoman")]
			Self::Yeoman => "Yeoman.gitignore",
			#[cfg(feature = "root-yii")]
			Self::Yii => "Yii.gitignore",
			#[cfg(feature = "root-zend-framework")]
			Self::ZendFramework => "ZendFramework.gitignore",
			#[cfg(feature = "root-zephir")]
			Self::Zephir => "Zephir.gitignore",
			#[cfg(feature = "root-zig")]
			Self::Zig => "Zig.gitignore",
		}
	}

	fn file_path(self) -> &'static str {
		match self {
			#[cfg(feature = "root-actionscript")]
			Self::Actionscript => "Actionscript.gitignore",
			#[cfg(feature = "root-ada")]
			Self::Ada => "Ada.gitignore",
			#[cfg(feature = "root-adventure-game-studio")]
			Self::AdventureGameStudio => "AdventureGameStudio.gitignore",
			#[cfg(feature = "root-agda")]
			Self::Agda => "Agda.gitignore",
			#[cfg(feature = "root-al")]
			Self::Al => "AL.gitignore",
			#[cfg(feature = "root-android")]
			Self::Android => "Android.gitignore",
			#[cfg(feature = "root-angular")]
			Self::Angular => "Angular.gitignore",
			#[cfg(feature = "root-app-engine")]
			Self::AppEngine => "AppEngine.gitignore",
			#[cfg(feature = "root-appcelerator-titanium")]
			Self::AppceleratorTitanium => "AppceleratorTitanium.gitignore",
			#[cfg(feature = "root-arch-linux-packages")]
			Self::ArchLinuxPackages => "ArchLinuxPackages.gitignore",
			#[cfg(feature = "root-autotools")]
			Self::Autotools => "Autotools.gitignore",
			#[cfg(feature = "root-ballerina")]
			Self::Ballerina => "Ballerina.gitignore",
			#[cfg(feature = "root-c")]
			Self::C => "C.gitignore",
			#[cfg(feature = "root-c-make")]
			Self::CMake => "CMake.gitignore",
			#[cfg(feature = "root-c-plus-plus")]
			Self::CPlusPlus => "C++.gitignore",
			#[cfg(feature = "root-cake-php")]
			Self::CakePhp => "CakePHP.gitignore",
			#[cfg(feature = "root-cf-wheels")]
			Self::CfWheels => "CFWheels.gitignore",
			#[cfg(feature = "root-chef-cookbook")]
			Self::ChefCookbook => "ChefCookbook.gitignore",
			#[cfg(feature = "root-clojure")]
			Self::Clojure => "Clojure.gitignore",
			#[cfg(feature = "root-code-igniter")]
			Self::CodeIgniter => "CodeIgniter.gitignore",
			#[cfg(feature = "root-common-lisp")]
			Self::CommonLisp => "CommonLisp.gitignore",
			#[cfg(feature = "root-composer")]
			Self::Composer => "Composer.gitignore",
			#[cfg(feature = "root-concrete5")]
			Self::Concrete5 => "Concrete5.gitignore",
			#[cfg(feature = "root-coq")]
			Self::Coq => "Coq.gitignore",
			#[cfg(feature = "root-craft-cms")]
			Self::CraftCms => "CraftCMS.gitignore",
			#[cfg(feature = "root-cuda")]
			Self::Cuda => "CUDA.gitignore",
			#[cfg(feature = "root-d")]
			Self::D => "D.gitignore",
			#[cfg(feature = "root-dart")]
			Self::Dart => "Dart.gitignore",
			#[cfg(feature = "root-delphi")]
			Self::Delphi => "Delphi.gitignore",
			#[cfg(feature = "root-dm")]
			Self::Dm => "DM.gitignore",
			#[cfg(feature = "root-dotnet")]
			Self::Dotnet => "Dotnet.gitignore",
			#[cfg(feature = "root-drupal")]
			Self::Drupal => "Drupal.gitignore",
			#[cfg(feature = "root-e-pi-server")]
			Self::EPiServer => "EPiServer.gitignore",
			#[cfg(feature = "root-eagle")]
			Self::Eagle => "Eagle.gitignore",
			#[cfg(feature = "root-ecutest")]
			Self::Ecutest => "ecu.test.gitignore",
			#[cfg(feature = "root-elisp")]
			Self::Elisp => "Elisp.gitignore",
			#[cfg(feature = "root-elixir")]
			Self::Elixir => "Elixir.gitignore",
			#[cfg(feature = "root-elm")]
			Self::Elm => "Elm.gitignore",
			#[cfg(feature = "root-erlang")]
			Self::Erlang => "Erlang.gitignore",
			#[cfg(feature = "root-expression-engine")]
			Self::ExpressionEngine => "ExpressionEngine.gitignore",
			#[cfg(feature = "root-ext-js")]
			Self::ExtJs => "ExtJs.gitignore",
			#[cfg(feature = "root-fancy")]
			Self::Fancy => "Fancy.gitignore",
			#[cfg(feature = "root-finale")]
			Self::Finale => "Finale.gitignore",
			#[cfg(feature = "root-firebase")]
			Self::Firebase => "Firebase.gitignore",
			#[cfg(feature = "root-flax-engine")]
			Self::FlaxEngine => "FlaxEngine.gitignore",
			#[cfg(feature = "root-flutter")]
			Self::Flutter => "Flutter.gitignore",
			#[cfg(feature = "root-force-dot-com")]
			Self::ForceDotCom => "ForceDotCom.gitignore",
			#[cfg(feature = "root-fortran")]
			Self::Fortran => "Fortran.gitignore",
			#[cfg(feature = "root-fuel-php")]
			Self::FuelPhp => "FuelPHP.gitignore",
			#[cfg(feature = "root-gcov")]
			Self::Gcov => "Gcov.gitignore",
			#[cfg(feature = "root-git-book")]
			Self::GitBook => "GitBook.gitignore",
			#[cfg(feature = "root-git-hub-pages")]
			Self::GitHubPages => "GitHubPages.gitignore",
			#[cfg(feature = "root-gleam")]
			Self::Gleam => "Gleam.gitignore",
			#[cfg(feature = "root-go")]
			Self::Go => "Go.gitignore",
			#[cfg(feature = "root-godot")]
			Self::Godot => "Godot.gitignore",
			#[cfg(feature = "root-gradle")]
			Self::Gradle => "Gradle.gitignore",
			#[cfg(feature = "root-grails")]
			Self::Grails => "Grails.gitignore",
			#[cfg(feature = "root-gwt")]
			Self::Gwt => "GWT.gitignore",
			#[cfg(feature = "root-haskell")]
			Self::Haskell => "Haskell.gitignore",
			#[cfg(feature = "root-haxe")]
			Self::Haxe => "Haxe.gitignore",
			#[cfg(feature = "root-hip")]
			Self::Hip => "HIP.gitignore",
			#[cfg(feature = "root-iar")]
			Self::Iar => "IAR.gitignore",
			#[cfg(feature = "root-idris")]
			Self::Idris => "Idris.gitignore",
			#[cfg(feature = "root-igor-pro")]
			Self::IgorPro => "IGORPro.gitignore",
			#[cfg(feature = "root-j-boss")]
			Self::JBoss => "JBoss.gitignore",
			#[cfg(feature = "root-java")]
			Self::Java => "Java.gitignore",
			#[cfg(feature = "root-jekyll")]
			Self::Jekyll => "Jekyll.gitignore",
			#[cfg(feature = "root-jenkins-home")]
			Self::JenkinsHome => "JENKINS_HOME.gitignore",
			#[cfg(feature = "root-joomla")]
			Self::Joomla => "Joomla.gitignore",
			#[cfg(feature = "root-julia")]
			Self::Julia => "Julia.gitignore",
			#[cfg(feature = "root-katalon")]
			Self::Katalon => "Katalon.gitignore",
			#[cfg(feature = "root-ki-cad")]
			Self::KiCad => "KiCad.gitignore",
			#[cfg(feature = "root-kohana")]
			Self::Kohana => "Kohana.gitignore",
			#[cfg(feature = "root-kotlin")]
			Self::Kotlin => "Kotlin.gitignore",
			#[cfg(feature = "root-lab-view")]
			Self::LabView => "LabVIEW.gitignore",
			#[cfg(feature = "root-lang-chain")]
			Self::LangChain => "LangChain.gitignore",
			#[cfg(feature = "root-laravel")]
			Self::Laravel => "Laravel.gitignore",
			#[cfg(feature = "root-leiningen")]
			Self::Leiningen => "Leiningen.gitignore",
			#[cfg(feature = "root-lemon-stand")]
			Self::LemonStand => "LemonStand.gitignore",
			#[cfg(feature = "root-lilypond")]
			Self::Lilypond => "Lilypond.gitignore",
			#[cfg(feature = "root-lithium")]
			Self::Lithium => "Lithium.gitignore",
			#[cfg(feature = "root-lua")]
			Self::Lua => "Lua.gitignore",
			#[cfg(feature = "root-luau")]
			Self::Luau => "Luau.gitignore",
			#[cfg(feature = "root-magento")]
			Self::Magento => "Magento.gitignore",
			#[cfg(feature = "root-maven")]
			Self::Maven => "Maven.gitignore",
			#[cfg(feature = "root-mercury")]
			Self::Mercury => "Mercury.gitignore",
			#[cfg(feature = "root-meta-programming-system")]
			Self::MetaProgrammingSystem => "MetaProgrammingSystem.gitignore",
			#[cfg(feature = "root-modelica")]
			Self::Modelica => "Modelica.gitignore",
			#[cfg(feature = "root-nanoc")]
			Self::Nanoc => "Nanoc.gitignore",
			#[cfg(feature = "root-nestjs")]
			Self::Nestjs => "Nestjs.gitignore",
			#[cfg(feature = "root-nextjs")]
			Self::Nextjs => "Nextjs.gitignore",
			#[cfg(feature = "root-nim")]
			Self::Nim => "Nim.gitignore",
			#[cfg(feature = "root-nix")]
			Self::Nix => "Nix.gitignore",
			#[cfg(feature = "root-node")]
			Self::Node => "Node.gitignore",
			#[cfg(feature = "root-o-caml")]
			Self::OCaml => "OCaml.gitignore",
			#[cfg(feature = "root-objective-c")]
			Self::ObjectiveC => "Objective-C.gitignore",
			#[cfg(feature = "root-opa")]
			Self::Opa => "Opa.gitignore",
			#[cfg(feature = "root-open-cart")]
			Self::OpenCart => "OpenCart.gitignore",
			#[cfg(feature = "root-oracle-forms")]
			Self::OracleForms => "OracleForms.gitignore",
			#[cfg(feature = "root-packer")]
			Self::Packer => "Packer.gitignore",
			#[cfg(feature = "root-perl")]
			Self::Perl => "Perl.gitignore",
			#[cfg(feature = "root-phalcon")]
			Self::Phalcon => "Phalcon.gitignore",
			#[cfg(feature = "root-play-framework")]
			Self::PlayFramework => "PlayFramework.gitignore",
			#[cfg(feature = "root-plone")]
			Self::Plone => "Plone.gitignore",
			#[cfg(feature = "root-prestashop")]
			Self::Prestashop => "Prestashop.gitignore",
			#[cfg(feature = "root-processing")]
			Self::Processing => "Processing.gitignore",
			#[cfg(feature = "root-pure-script")]
			Self::PureScript => "PureScript.gitignore",
			#[cfg(feature = "root-python")]
			Self::Python => "Python.gitignore",
			#[cfg(feature = "root-qooxdoo")]
			Self::Qooxdoo => "Qooxdoo.gitignore",
			#[cfg(feature = "root-qt")]
			Self::Qt => "Qt.gitignore",
			#[cfg(feature = "root-r")]
			Self::R => "R.gitignore",
			#[cfg(feature = "root-racket")]
			Self::Racket => "Racket.gitignore",
			#[cfg(feature = "root-rails")]
			Self::Rails => "Rails.gitignore",
			#[cfg(feature = "root-raku")]
			Self::Raku => "Raku.gitignore",
			#[cfg(feature = "root-re-script")]
			Self::ReScript => "ReScript.gitignore",
			#[cfg(feature = "root-rhodes-rhomobile")]
			Self::RhodesRhomobile => "RhodesRhomobile.gitignore",
			#[cfg(feature = "root-ros")]
			Self::Ros => "ROS.gitignore",
			#[cfg(feature = "root-ruby")]
			Self::Ruby => "Ruby.gitignore",
			#[cfg(feature = "root-rust")]
			Self::Rust => "Rust.gitignore",
			#[cfg(feature = "root-s-cons")]
			Self::SCons => "SCons.gitignore",
			#[cfg(feature = "root-sass")]
			Self::Sass => "Sass.gitignore",
			#[cfg(feature = "root-scala")]
			Self::Scala => "Scala.gitignore",
			#[cfg(feature = "root-scheme")]
			Self::Scheme => "Scheme.gitignore",
			#[cfg(feature = "root-scrivener")]
			Self::Scrivener => "Scrivener.gitignore",
			#[cfg(feature = "root-sdcc")]
			Self::Sdcc => "Sdcc.gitignore",
			#[cfg(feature = "root-seam-gen")]
			Self::SeamGen => "SeamGen.gitignore",
			#[cfg(feature = "root-sketch-up")]
			Self::SketchUp => "SketchUp.gitignore",
			#[cfg(feature = "root-smalltalk")]
			Self::Smalltalk => "Smalltalk.gitignore",
			#[cfg(feature = "root-solidity-remix")]
			Self::SolidityRemix => "Solidity-Remix.gitignore",
			#[cfg(feature = "root-ssdt-sqlproj")]
			Self::SsdtSqlproj => "SSDT-sqlproj.gitignore",
			#[cfg(feature = "root-stella")]
			Self::Stella => "Stella.gitignore",
			#[cfg(feature = "root-sugar-crm")]
			Self::SugarCrm => "SugarCRM.gitignore",
			#[cfg(feature = "root-swift")]
			Self::Swift => "Swift.gitignore",
			#[cfg(feature = "root-symfony")]
			Self::Symfony => "Symfony.gitignore",
			#[cfg(feature = "root-symphony-cms")]
			Self::SymphonyCms => "SymphonyCMS.gitignore",
			#[cfg(feature = "root-te-x")]
			Self::TeX => "TeX.gitignore",
			#[cfg(feature = "root-terraform")]
			Self::Terraform => "Terraform.gitignore",
			#[cfg(feature = "root-test-complete")]
			Self::TestComplete => "TestComplete.gitignore",
			#[cfg(feature = "root-textpattern")]
			Self::Textpattern => "Textpattern.gitignore",
			#[cfg(feature = "root-turbo-gears2")]
			Self::TurboGears2 => "TurboGears2.gitignore",
			#[cfg(feature = "root-twin-cat3")]
			Self::TwinCat3 => "TwinCAT3.gitignore",
			#[cfg(feature = "root-typo3")]
			Self::Typo3 => "Typo3.gitignore",
			#[cfg(feature = "root-unity")]
			Self::Unity => "Unity.gitignore",
			#[cfg(feature = "root-unreal-engine")]
			Self::UnrealEngine => "UnrealEngine.gitignore",
			#[cfg(feature = "root-vba")]
			Self::Vba => "VBA.gitignore",
			#[cfg(feature = "root-visual-studio")]
			Self::VisualStudio => "VisualStudio.gitignore",
			#[cfg(feature = "root-vvvv")]
			Self::Vvvv => "VVVV.gitignore",
			#[cfg(feature = "root-waf")]
			Self::Waf => "Waf.gitignore",
			#[cfg(feature = "root-word-press")]
			Self::WordPress => "WordPress.gitignore",
			#[cfg(feature = "root-xojo")]
			Self::Xojo => "Xojo.gitignore",
			#[cfg(feature = "root-yeoman")]
			Self::Yeoman => "Yeoman.gitignore",
			#[cfg(feature = "root-yii")]
			Self::Yii => "Yii.gitignore",
			#[cfg(feature = "root-zend-framework")]
			Self::ZendFramework => "ZendFramework.gitignore",
			#[cfg(feature = "root-zephir")]
			Self::Zephir => "Zephir.gitignore",
			#[cfg(feature = "root-zig")]
			Self::Zig => "Zig.gitignore",
		}
	}

	#[cfg(feature = "std")]
	fn list() -> Vec<&'static str> {
		#[allow(unused_mut)]
		let mut list = Vec::with_capacity(155);
		#[cfg(feature = "root-actionscript")]
		list.push("Actionscript");
		#[cfg(feature = "root-ada")]
		list.push("Ada");
		#[cfg(feature = "root-adventure-game-studio")]
		list.push("AdventureGameStudio");
		#[cfg(feature = "root-agda")]
		list.push("Agda");
		#[cfg(feature = "root-al")]
		list.push("Al");
		#[cfg(feature = "root-android")]
		list.push("Android");
		#[cfg(feature = "root-angular")]
		list.push("Angular");
		#[cfg(feature = "root-app-engine")]
		list.push("AppEngine");
		#[cfg(feature = "root-appcelerator-titanium")]
		list.push("AppceleratorTitanium");
		#[cfg(feature = "root-arch-linux-packages")]
		list.push("ArchLinuxPackages");
		#[cfg(feature = "root-autotools")]
		list.push("Autotools");
		#[cfg(feature = "root-ballerina")]
		list.push("Ballerina");
		#[cfg(feature = "root-c")]
		list.push("C");
		#[cfg(feature = "root-c-make")]
		list.push("CMake");
		#[cfg(feature = "root-c-plus-plus")]
		list.push("CPlusPlus");
		#[cfg(feature = "root-cake-php")]
		list.push("CakePhp");
		#[cfg(feature = "root-cf-wheels")]
		list.push("CfWheels");
		#[cfg(feature = "root-chef-cookbook")]
		list.push("ChefCookbook");
		#[cfg(feature = "root-clojure")]
		list.push("Clojure");
		#[cfg(feature = "root-code-igniter")]
		list.push("CodeIgniter");
		#[cfg(feature = "root-common-lisp")]
		list.push("CommonLisp");
		#[cfg(feature = "root-composer")]
		list.push("Composer");
		#[cfg(feature = "root-concrete5")]
		list.push("Concrete5");
		#[cfg(feature = "root-coq")]
		list.push("Coq");
		#[cfg(feature = "root-craft-cms")]
		list.push("CraftCms");
		#[cfg(feature = "root-cuda")]
		list.push("Cuda");
		#[cfg(feature = "root-d")]
		list.push("D");
		#[cfg(feature = "root-dart")]
		list.push("Dart");
		#[cfg(feature = "root-delphi")]
		list.push("Delphi");
		#[cfg(feature = "root-dm")]
		list.push("Dm");
		#[cfg(feature = "root-dotnet")]
		list.push("Dotnet");
		#[cfg(feature = "root-drupal")]
		list.push("Drupal");
		#[cfg(feature = "root-e-pi-server")]
		list.push("EPiServer");
		#[cfg(feature = "root-eagle")]
		list.push("Eagle");
		#[cfg(feature = "root-ecutest")]
		list.push("Ecutest");
		#[cfg(feature = "root-elisp")]
		list.push("Elisp");
		#[cfg(feature = "root-elixir")]
		list.push("Elixir");
		#[cfg(feature = "root-elm")]
		list.push("Elm");
		#[cfg(feature = "root-erlang")]
		list.push("Erlang");
		#[cfg(feature = "root-expression-engine")]
		list.push("ExpressionEngine");
		#[cfg(feature = "root-ext-js")]
		list.push("ExtJs");
		#[cfg(feature = "root-fancy")]
		list.push("Fancy");
		#[cfg(feature = "root-finale")]
		list.push("Finale");
		#[cfg(feature = "root-firebase")]
		list.push("Firebase");
		#[cfg(feature = "root-flax-engine")]
		list.push("FlaxEngine");
		#[cfg(feature = "root-flutter")]
		list.push("Flutter");
		#[cfg(feature = "root-force-dot-com")]
		list.push("ForceDotCom");
		#[cfg(feature = "root-fortran")]
		list.push("Fortran");
		#[cfg(feature = "root-fuel-php")]
		list.push("FuelPhp");
		#[cfg(feature = "root-gcov")]
		list.push("Gcov");
		#[cfg(feature = "root-git-book")]
		list.push("GitBook");
		#[cfg(feature = "root-git-hub-pages")]
		list.push("GitHubPages");
		#[cfg(feature = "root-gleam")]
		list.push("Gleam");
		#[cfg(feature = "root-go")]
		list.push("Go");
		#[cfg(feature = "root-godot")]
		list.push("Godot");
		#[cfg(feature = "root-gradle")]
		list.push("Gradle");
		#[cfg(feature = "root-grails")]
		list.push("Grails");
		#[cfg(feature = "root-gwt")]
		list.push("Gwt");
		#[cfg(feature = "root-haskell")]
		list.push("Haskell");
		#[cfg(feature = "root-haxe")]
		list.push("Haxe");
		#[cfg(feature = "root-hip")]
		list.push("Hip");
		#[cfg(feature = "root-iar")]
		list.push("Iar");
		#[cfg(feature = "root-idris")]
		list.push("Idris");
		#[cfg(feature = "root-igor-pro")]
		list.push("IgorPro");
		#[cfg(feature = "root-j-boss")]
		list.push("JBoss");
		#[cfg(feature = "root-java")]
		list.push("Java");
		#[cfg(feature = "root-jekyll")]
		list.push("Jekyll");
		#[cfg(feature = "root-jenkins-home")]
		list.push("JenkinsHome");
		#[cfg(feature = "root-joomla")]
		list.push("Joomla");
		#[cfg(feature = "root-julia")]
		list.push("Julia");
		#[cfg(feature = "root-katalon")]
		list.push("Katalon");
		#[cfg(feature = "root-ki-cad")]
		list.push("KiCad");
		#[cfg(feature = "root-kohana")]
		list.push("Kohana");
		#[cfg(feature = "root-kotlin")]
		list.push("Kotlin");
		#[cfg(feature = "root-lab-view")]
		list.push("LabView");
		#[cfg(feature = "root-lang-chain")]
		list.push("LangChain");
		#[cfg(feature = "root-laravel")]
		list.push("Laravel");
		#[cfg(feature = "root-leiningen")]
		list.push("Leiningen");
		#[cfg(feature = "root-lemon-stand")]
		list.push("LemonStand");
		#[cfg(feature = "root-lilypond")]
		list.push("Lilypond");
		#[cfg(feature = "root-lithium")]
		list.push("Lithium");
		#[cfg(feature = "root-lua")]
		list.push("Lua");
		#[cfg(feature = "root-luau")]
		list.push("Luau");
		#[cfg(feature = "root-magento")]
		list.push("Magento");
		#[cfg(feature = "root-maven")]
		list.push("Maven");
		#[cfg(feature = "root-mercury")]
		list.push("Mercury");
		#[cfg(feature = "root-meta-programming-system")]
		list.push("MetaProgrammingSystem");
		#[cfg(feature = "root-modelica")]
		list.push("Modelica");
		#[cfg(feature = "root-nanoc")]
		list.push("Nanoc");
		#[cfg(feature = "root-nestjs")]
		list.push("Nestjs");
		#[cfg(feature = "root-nextjs")]
		list.push("Nextjs");
		#[cfg(feature = "root-nim")]
		list.push("Nim");
		#[cfg(feature = "root-nix")]
		list.push("Nix");
		#[cfg(feature = "root-node")]
		list.push("Node");
		#[cfg(feature = "root-o-caml")]
		list.push("OCaml");
		#[cfg(feature = "root-objective-c")]
		list.push("ObjectiveC");
		#[cfg(feature = "root-opa")]
		list.push("Opa");
		#[cfg(feature = "root-open-cart")]
		list.push("OpenCart");
		#[cfg(feature = "root-oracle-forms")]
		list.push("OracleForms");
		#[cfg(feature = "root-packer")]
		list.push("Packer");
		#[cfg(feature = "root-perl")]
		list.push("Perl");
		#[cfg(feature = "root-phalcon")]
		list.push("Phalcon");
		#[cfg(feature = "root-play-framework")]
		list.push("PlayFramework");
		#[cfg(feature = "root-plone")]
		list.push("Plone");
		#[cfg(feature = "root-prestashop")]
		list.push("Prestashop");
		#[cfg(feature = "root-processing")]
		list.push("Processing");
		#[cfg(feature = "root-pure-script")]
		list.push("PureScript");
		#[cfg(feature = "root-python")]
		list.push("Python");
		#[cfg(feature = "root-qooxdoo")]
		list.push("Qooxdoo");
		#[cfg(feature = "root-qt")]
		list.push("Qt");
		#[cfg(feature = "root-r")]
		list.push("R");
		#[cfg(feature = "root-racket")]
		list.push("Racket");
		#[cfg(feature = "root-rails")]
		list.push("Rails");
		#[cfg(feature = "root-raku")]
		list.push("Raku");
		#[cfg(feature = "root-re-script")]
		list.push("ReScript");
		#[cfg(feature = "root-rhodes-rhomobile")]
		list.push("RhodesRhomobile");
		#[cfg(feature = "root-ros")]
		list.push("Ros");
		#[cfg(feature = "root-ruby")]
		list.push("Ruby");
		#[cfg(feature = "root-rust")]
		list.push("Rust");
		#[cfg(feature = "root-s-cons")]
		list.push("SCons");
		#[cfg(feature = "root-sass")]
		list.push("Sass");
		#[cfg(feature = "root-scala")]
		list.push("Scala");
		#[cfg(feature = "root-scheme")]
		list.push("Scheme");
		#[cfg(feature = "root-scrivener")]
		list.push("Scrivener");
		#[cfg(feature = "root-sdcc")]
		list.push("Sdcc");
		#[cfg(feature = "root-seam-gen")]
		list.push("SeamGen");
		#[cfg(feature = "root-sketch-up")]
		list.push("SketchUp");
		#[cfg(feature = "root-smalltalk")]
		list.push("Smalltalk");
		#[cfg(feature = "root-solidity-remix")]
		list.push("SolidityRemix");
		#[cfg(feature = "root-ssdt-sqlproj")]
		list.push("SsdtSqlproj");
		#[cfg(feature = "root-stella")]
		list.push("Stella");
		#[cfg(feature = "root-sugar-crm")]
		list.push("SugarCrm");
		#[cfg(feature = "root-swift")]
		list.push("Swift");
		#[cfg(feature = "root-symfony")]
		list.push("Symfony");
		#[cfg(feature = "root-symphony-cms")]
		list.push("SymphonyCms");
		#[cfg(feature = "root-te-x")]
		list.push("TeX");
		#[cfg(feature = "root-terraform")]
		list.push("Terraform");
		#[cfg(feature = "root-test-complete")]
		list.push("TestComplete");
		#[cfg(feature = "root-textpattern")]
		list.push("Textpattern");
		#[cfg(feature = "root-turbo-gears2")]
		list.push("TurboGears2");
		#[cfg(feature = "root-twin-cat3")]
		list.push("TwinCat3");
		#[cfg(feature = "root-typo3")]
		list.push("Typo3");
		#[cfg(feature = "root-unity")]
		list.push("Unity");
		#[cfg(feature = "root-unreal-engine")]
		list.push("UnrealEngine");
		#[cfg(feature = "root-vba")]
		list.push("Vba");
		#[cfg(feature = "root-visual-studio")]
		list.push("VisualStudio");
		#[cfg(feature = "root-vvvv")]
		list.push("Vvvv");
		#[cfg(feature = "root-waf")]
		list.push("Waf");
		#[cfg(feature = "root-word-press")]
		list.push("WordPress");
		#[cfg(feature = "root-xojo")]
		list.push("Xojo");
		#[cfg(feature = "root-yeoman")]
		list.push("Yeoman");
		#[cfg(feature = "root-yii")]
		list.push("Yii");
		#[cfg(feature = "root-zend-framework")]
		list.push("ZendFramework");
		#[cfg(feature = "root-zephir")]
		list.push("Zephir");
		#[cfg(feature = "root-zig")]
		list.push("Zig");
		list
	}

	fn get(variant: &'static str) -> Option<Self> {
		#[allow(unreachable_code)]
		Some(match variant {
			#[cfg(feature = "root-actionscript")]
			"Actionscript" => Self::Actionscript,
			#[cfg(feature = "root-ada")]
			"Ada" => Self::Ada,
			#[cfg(feature = "root-adventure-game-studio")]
			"AdventureGameStudio" => Self::AdventureGameStudio,
			#[cfg(feature = "root-agda")]
			"Agda" => Self::Agda,
			#[cfg(feature = "root-al")]
			"Al" => Self::Al,
			#[cfg(feature = "root-android")]
			"Android" => Self::Android,
			#[cfg(feature = "root-angular")]
			"Angular" => Self::Angular,
			#[cfg(feature = "root-app-engine")]
			"AppEngine" => Self::AppEngine,
			#[cfg(feature = "root-appcelerator-titanium")]
			"AppceleratorTitanium" => Self::AppceleratorTitanium,
			#[cfg(feature = "root-arch-linux-packages")]
			"ArchLinuxPackages" => Self::ArchLinuxPackages,
			#[cfg(feature = "root-autotools")]
			"Autotools" => Self::Autotools,
			#[cfg(feature = "root-ballerina")]
			"Ballerina" => Self::Ballerina,
			#[cfg(feature = "root-c")]
			"C" => Self::C,
			#[cfg(feature = "root-c-make")]
			"CMake" => Self::CMake,
			#[cfg(feature = "root-c-plus-plus")]
			"CPlusPlus" => Self::CPlusPlus,
			#[cfg(feature = "root-cake-php")]
			"CakePhp" => Self::CakePhp,
			#[cfg(feature = "root-cf-wheels")]
			"CfWheels" => Self::CfWheels,
			#[cfg(feature = "root-chef-cookbook")]
			"ChefCookbook" => Self::ChefCookbook,
			#[cfg(feature = "root-clojure")]
			"Clojure" => Self::Clojure,
			#[cfg(feature = "root-code-igniter")]
			"CodeIgniter" => Self::CodeIgniter,
			#[cfg(feature = "root-common-lisp")]
			"CommonLisp" => Self::CommonLisp,
			#[cfg(feature = "root-composer")]
			"Composer" => Self::Composer,
			#[cfg(feature = "root-concrete5")]
			"Concrete5" => Self::Concrete5,
			#[cfg(feature = "root-coq")]
			"Coq" => Self::Coq,
			#[cfg(feature = "root-craft-cms")]
			"CraftCms" => Self::CraftCms,
			#[cfg(feature = "root-cuda")]
			"Cuda" => Self::Cuda,
			#[cfg(feature = "root-d")]
			"D" => Self::D,
			#[cfg(feature = "root-dart")]
			"Dart" => Self::Dart,
			#[cfg(feature = "root-delphi")]
			"Delphi" => Self::Delphi,
			#[cfg(feature = "root-dm")]
			"Dm" => Self::Dm,
			#[cfg(feature = "root-dotnet")]
			"Dotnet" => Self::Dotnet,
			#[cfg(feature = "root-drupal")]
			"Drupal" => Self::Drupal,
			#[cfg(feature = "root-e-pi-server")]
			"EPiServer" => Self::EPiServer,
			#[cfg(feature = "root-eagle")]
			"Eagle" => Self::Eagle,
			#[cfg(feature = "root-ecutest")]
			"Ecutest" => Self::Ecutest,
			#[cfg(feature = "root-elisp")]
			"Elisp" => Self::Elisp,
			#[cfg(feature = "root-elixir")]
			"Elixir" => Self::Elixir,
			#[cfg(feature = "root-elm")]
			"Elm" => Self::Elm,
			#[cfg(feature = "root-erlang")]
			"Erlang" => Self::Erlang,
			#[cfg(feature = "root-expression-engine")]
			"ExpressionEngine" => Self::ExpressionEngine,
			#[cfg(feature = "root-ext-js")]
			"ExtJs" => Self::ExtJs,
			#[cfg(feature = "root-fancy")]
			"Fancy" => Self::Fancy,
			#[cfg(feature = "root-finale")]
			"Finale" => Self::Finale,
			#[cfg(feature = "root-firebase")]
			"Firebase" => Self::Firebase,
			#[cfg(feature = "root-flax-engine")]
			"FlaxEngine" => Self::FlaxEngine,
			#[cfg(feature = "root-flutter")]
			"Flutter" => Self::Flutter,
			#[cfg(feature = "root-force-dot-com")]
			"ForceDotCom" => Self::ForceDotCom,
			#[cfg(feature = "root-fortran")]
			"Fortran" => Self::Fortran,
			#[cfg(feature = "root-fuel-php")]
			"FuelPhp" => Self::FuelPhp,
			#[cfg(feature = "root-gcov")]
			"Gcov" => Self::Gcov,
			#[cfg(feature = "root-git-book")]
			"GitBook" => Self::GitBook,
			#[cfg(feature = "root-git-hub-pages")]
			"GitHubPages" => Self::GitHubPages,
			#[cfg(feature = "root-gleam")]
			"Gleam" => Self::Gleam,
			#[cfg(feature = "root-go")]
			"Go" => Self::Go,
			#[cfg(feature = "root-godot")]
			"Godot" => Self::Godot,
			#[cfg(feature = "root-gradle")]
			"Gradle" => Self::Gradle,
			#[cfg(feature = "root-grails")]
			"Grails" => Self::Grails,
			#[cfg(feature = "root-gwt")]
			"Gwt" => Self::Gwt,
			#[cfg(feature = "root-haskell")]
			"Haskell" => Self::Haskell,
			#[cfg(feature = "root-haxe")]
			"Haxe" => Self::Haxe,
			#[cfg(feature = "root-hip")]
			"Hip" => Self::Hip,
			#[cfg(feature = "root-iar")]
			"Iar" => Self::Iar,
			#[cfg(feature = "root-idris")]
			"Idris" => Self::Idris,
			#[cfg(feature = "root-igor-pro")]
			"IgorPro" => Self::IgorPro,
			#[cfg(feature = "root-j-boss")]
			"JBoss" => Self::JBoss,
			#[cfg(feature = "root-java")]
			"Java" => Self::Java,
			#[cfg(feature = "root-jekyll")]
			"Jekyll" => Self::Jekyll,
			#[cfg(feature = "root-jenkins-home")]
			"JenkinsHome" => Self::JenkinsHome,
			#[cfg(feature = "root-joomla")]
			"Joomla" => Self::Joomla,
			#[cfg(feature = "root-julia")]
			"Julia" => Self::Julia,
			#[cfg(feature = "root-katalon")]
			"Katalon" => Self::Katalon,
			#[cfg(feature = "root-ki-cad")]
			"KiCad" => Self::KiCad,
			#[cfg(feature = "root-kohana")]
			"Kohana" => Self::Kohana,
			#[cfg(feature = "root-kotlin")]
			"Kotlin" => Self::Kotlin,
			#[cfg(feature = "root-lab-view")]
			"LabView" => Self::LabView,
			#[cfg(feature = "root-lang-chain")]
			"LangChain" => Self::LangChain,
			#[cfg(feature = "root-laravel")]
			"Laravel" => Self::Laravel,
			#[cfg(feature = "root-leiningen")]
			"Leiningen" => Self::Leiningen,
			#[cfg(feature = "root-lemon-stand")]
			"LemonStand" => Self::LemonStand,
			#[cfg(feature = "root-lilypond")]
			"Lilypond" => Self::Lilypond,
			#[cfg(feature = "root-lithium")]
			"Lithium" => Self::Lithium,
			#[cfg(feature = "root-lua")]
			"Lua" => Self::Lua,
			#[cfg(feature = "root-luau")]
			"Luau" => Self::Luau,
			#[cfg(feature = "root-magento")]
			"Magento" => Self::Magento,
			#[cfg(feature = "root-maven")]
			"Maven" => Self::Maven,
			#[cfg(feature = "root-mercury")]
			"Mercury" => Self::Mercury,
			#[cfg(feature = "root-meta-programming-system")]
			"MetaProgrammingSystem" => Self::MetaProgrammingSystem,
			#[cfg(feature = "root-modelica")]
			"Modelica" => Self::Modelica,
			#[cfg(feature = "root-nanoc")]
			"Nanoc" => Self::Nanoc,
			#[cfg(feature = "root-nestjs")]
			"Nestjs" => Self::Nestjs,
			#[cfg(feature = "root-nextjs")]
			"Nextjs" => Self::Nextjs,
			#[cfg(feature = "root-nim")]
			"Nim" => Self::Nim,
			#[cfg(feature = "root-nix")]
			"Nix" => Self::Nix,
			#[cfg(feature = "root-node")]
			"Node" => Self::Node,
			#[cfg(feature = "root-o-caml")]
			"OCaml" => Self::OCaml,
			#[cfg(feature = "root-objective-c")]
			"ObjectiveC" => Self::ObjectiveC,
			#[cfg(feature = "root-opa")]
			"Opa" => Self::Opa,
			#[cfg(feature = "root-open-cart")]
			"OpenCart" => Self::OpenCart,
			#[cfg(feature = "root-oracle-forms")]
			"OracleForms" => Self::OracleForms,
			#[cfg(feature = "root-packer")]
			"Packer" => Self::Packer,
			#[cfg(feature = "root-perl")]
			"Perl" => Self::Perl,
			#[cfg(feature = "root-phalcon")]
			"Phalcon" => Self::Phalcon,
			#[cfg(feature = "root-play-framework")]
			"PlayFramework" => Self::PlayFramework,
			#[cfg(feature = "root-plone")]
			"Plone" => Self::Plone,
			#[cfg(feature = "root-prestashop")]
			"Prestashop" => Self::Prestashop,
			#[cfg(feature = "root-processing")]
			"Processing" => Self::Processing,
			#[cfg(feature = "root-pure-script")]
			"PureScript" => Self::PureScript,
			#[cfg(feature = "root-python")]
			"Python" => Self::Python,
			#[cfg(feature = "root-qooxdoo")]
			"Qooxdoo" => Self::Qooxdoo,
			#[cfg(feature = "root-qt")]
			"Qt" => Self::Qt,
			#[cfg(feature = "root-r")]
			"R" => Self::R,
			#[cfg(feature = "root-racket")]
			"Racket" => Self::Racket,
			#[cfg(feature = "root-rails")]
			"Rails" => Self::Rails,
			#[cfg(feature = "root-raku")]
			"Raku" => Self::Raku,
			#[cfg(feature = "root-re-script")]
			"ReScript" => Self::ReScript,
			#[cfg(feature = "root-rhodes-rhomobile")]
			"RhodesRhomobile" => Self::RhodesRhomobile,
			#[cfg(feature = "root-ros")]
			"Ros" => Self::Ros,
			#[cfg(feature = "root-ruby")]
			"Ruby" => Self::Ruby,
			#[cfg(feature = "root-rust")]
			"Rust" => Self::Rust,
			#[cfg(feature = "root-s-cons")]
			"SCons" => Self::SCons,
			#[cfg(feature = "root-sass")]
			"Sass" => Self::Sass,
			#[cfg(feature = "root-scala")]
			"Scala" => Self::Scala,
			#[cfg(feature = "root-scheme")]
			"Scheme" => Self::Scheme,
			#[cfg(feature = "root-scrivener")]
			"Scrivener" => Self::Scrivener,
			#[cfg(feature = "root-sdcc")]
			"Sdcc" => Self::Sdcc,
			#[cfg(feature = "root-seam-gen")]
			"SeamGen" => Self::SeamGen,
			#[cfg(feature = "root-sketch-up")]
			"SketchUp" => Self::SketchUp,
			#[cfg(feature = "root-smalltalk")]
			"Smalltalk" => Self::Smalltalk,
			#[cfg(feature = "root-solidity-remix")]
			"SolidityRemix" => Self::SolidityRemix,
			#[cfg(feature = "root-ssdt-sqlproj")]
			"SsdtSqlproj" => Self::SsdtSqlproj,
			#[cfg(feature = "root-stella")]
			"Stella" => Self::Stella,
			#[cfg(feature = "root-sugar-crm")]
			"SugarCrm" => Self::SugarCrm,
			#[cfg(feature = "root-swift")]
			"Swift" => Self::Swift,
			#[cfg(feature = "root-symfony")]
			"Symfony" => Self::Symfony,
			#[cfg(feature = "root-symphony-cms")]
			"SymphonyCms" => Self::SymphonyCms,
			#[cfg(feature = "root-te-x")]
			"TeX" => Self::TeX,
			#[cfg(feature = "root-terraform")]
			"Terraform" => Self::Terraform,
			#[cfg(feature = "root-test-complete")]
			"TestComplete" => Self::TestComplete,
			#[cfg(feature = "root-textpattern")]
			"Textpattern" => Self::Textpattern,
			#[cfg(feature = "root-turbo-gears2")]
			"TurboGears2" => Self::TurboGears2,
			#[cfg(feature = "root-twin-cat3")]
			"TwinCat3" => Self::TwinCat3,
			#[cfg(feature = "root-typo3")]
			"Typo3" => Self::Typo3,
			#[cfg(feature = "root-unity")]
			"Unity" => Self::Unity,
			#[cfg(feature = "root-unreal-engine")]
			"UnrealEngine" => Self::UnrealEngine,
			#[cfg(feature = "root-vba")]
			"Vba" => Self::Vba,
			#[cfg(feature = "root-visual-studio")]
			"VisualStudio" => Self::VisualStudio,
			#[cfg(feature = "root-vvvv")]
			"Vvvv" => Self::Vvvv,
			#[cfg(feature = "root-waf")]
			"Waf" => Self::Waf,
			#[cfg(feature = "root-word-press")]
			"WordPress" => Self::WordPress,
			#[cfg(feature = "root-xojo")]
			"Xojo" => Self::Xojo,
			#[cfg(feature = "root-yeoman")]
			"Yeoman" => Self::Yeoman,
			#[cfg(feature = "root-yii")]
			"Yii" => Self::Yii,
			#[cfg(feature = "root-zend-framework")]
			"ZendFramework" => Self::ZendFramework,
			#[cfg(feature = "root-zephir")]
			"Zephir" => Self::Zephir,
			#[cfg(feature = "root-zig")]
			"Zig" => Self::Zig,
			_ => return None,
		})
	}
}

#[cfg(all(feature = "std", not(feature = "no-contents")))]
impl std::fmt::Display for Root {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.contents())
	}
}
