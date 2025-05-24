
use crate::GitIgnore;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Global {
	#[cfg(feature = "global-al")]
	Al,
	#[cfg(feature = "global-anjuta")]
	Anjuta,
	#[cfg(feature = "global-ansible")]
	Ansible,
	#[cfg(feature = "global-archives")]
	Archives,
	#[cfg(feature = "global-backup")]
	Backup,
	#[cfg(feature = "global-bazaar")]
	Bazaar,
	#[cfg(feature = "global-bricx-cc")]
	BricxCc,
	#[cfg(feature = "global-calabash")]
	Calabash,
	#[cfg(feature = "global-cloud9")]
	Cloud9,
	#[cfg(feature = "global-code-kit")]
	CodeKit,
	#[cfg(feature = "global-cursor")]
	Cursor,
	#[cfg(feature = "global-cvs")]
	Cvs,
	#[cfg(feature = "global-dart-editor")]
	DartEditor,
	#[cfg(feature = "global-diff")]
	Diff,
	#[cfg(feature = "global-dreamweaver")]
	Dreamweaver,
	#[cfg(feature = "global-dropbox")]
	Dropbox,
	#[cfg(feature = "global-eclipse")]
	Eclipse,
	#[cfg(feature = "global-eiffel-studio")]
	EiffelStudio,
	#[cfg(feature = "global-emacs")]
	Emacs,
	#[cfg(feature = "global-ensime")]
	Ensime,
	#[cfg(feature = "global-espresso")]
	Espresso,
	#[cfg(feature = "global-flex-builder")]
	FlexBuilder,
	#[cfg(feature = "global-gpg")]
	Gpg,
	#[cfg(feature = "global-images")]
	Images,
	#[cfg(feature = "global-j-developer")]
	JDeveloper,
	#[cfg(feature = "global-j-env")]
	JEnv,
	#[cfg(feature = "global-jet-brains")]
	JetBrains,
	#[cfg(feature = "global-k-develop4")]
	KDevelop4,
	#[cfg(feature = "global-kate")]
	Kate,
	#[cfg(feature = "global-lazarus")]
	Lazarus,
	#[cfg(feature = "global-libre-office")]
	LibreOffice,
	#[cfg(feature = "global-linux")]
	Linux,
	#[cfg(feature = "global-ly-x")]
	LyX,
	#[cfg(feature = "global-mac-os")]
	MacOs,
	#[cfg(feature = "global-matlab")]
	Matlab,
	#[cfg(feature = "global-mercurial")]
	Mercurial,
	#[cfg(feature = "global-metals")]
	Metals,
	#[cfg(feature = "global-microsoft-office")]
	MicrosoftOffice,
	#[cfg(feature = "global-model-sim")]
	ModelSim,
	#[cfg(feature = "global-momentics")]
	Momentics,
	#[cfg(feature = "global-mono-develop")]
	MonoDevelop,
	#[cfg(feature = "global-net-beans")]
	NetBeans,
	#[cfg(feature = "global-ninja")]
	Ninja,
	#[cfg(feature = "global-notepad-pp")]
	NotepadPp,
	#[cfg(feature = "global-octave")]
	Octave,
	#[cfg(feature = "global-otto")]
	Otto,
	#[cfg(feature = "global-p-so-c-creator")]
	PSoCCreator,
	#[cfg(feature = "global-patch")]
	Patch,
	#[cfg(feature = "global-platform-io")]
	PlatformIo,
	#[cfg(feature = "global-pu-tty")]
	PuTty,
	#[cfg(feature = "global-redcar")]
	Redcar,
	#[cfg(feature = "global-redis")]
	Redis,
	#[cfg(feature = "global-sbt")]
	Sbt,
	#[cfg(feature = "global-slick-edit")]
	SlickEdit,
	#[cfg(feature = "global-stata")]
	Stata,
	#[cfg(feature = "global-sublime-text")]
	SublimeText,
	#[cfg(feature = "global-svn")]
	Svn,
	#[cfg(feature = "global-syncthing")]
	Syncthing,
	#[cfg(feature = "global-synopsys-vcs")]
	SynopsysVcs,
	#[cfg(feature = "global-tags")]
	Tags,
	#[cfg(feature = "global-text-mate")]
	TextMate,
	#[cfg(feature = "global-tortoise-git")]
	TortoiseGit,
	#[cfg(feature = "global-vagrant")]
	Vagrant,
	#[cfg(feature = "global-vim")]
	Vim,
	#[cfg(feature = "global-virtual-env")]
	VirtualEnv,
	#[cfg(feature = "global-virtuoso")]
	Virtuoso,
	#[cfg(feature = "global-visual-studio-code")]
	VisualStudioCode,
	#[cfg(feature = "global-web-methods")]
	WebMethods,
	#[cfg(feature = "global-windows")]
	Windows,
	#[cfg(feature = "global-xcode")]
	Xcode,
	#[cfg(feature = "global-xilinx-ise")]
	XilinxIse,
}

impl GitIgnore for Global {
	#[cfg(feature = "no-contents")]
	fn contents(self) -> &'static str {
		""
	}

	#[cfg(not(feature = "no-contents"))]
	fn contents(self) -> &'static str {
		match self { #[cfg(feature = "global-al")] Self::Al => ".vscode/*\n!.vscode/settings.json\n!.vscode/tasks.json\n!.vscode/launch.json\n!.vscode/extensions.json\n*.code-workspace\n\n# Local History for Visual Studio Code\n.history/\n*.app\n.snapshots/*\n", #[cfg(feature = "global-anjuta")] Self::Anjuta => "# Local configuration folder and symbol database\n/.anjuta/\n/.anjuta_sym_db.db\n", #[cfg(feature = "global-ansible")] Self::Ansible => "*.retry\n.ansible/\n", #[cfg(feature = "global-archives")] Self::Archives => "# It's better to unpack these files and commit the raw source because\n# git has its own built in compression methods.\n*.7z\n*.jar\n*.rar\n*.zip\n*.gz\n*.gzip\n*.tgz\n*.bzip\n*.bzip2\n*.bz2\n*.xz\n*.lzma\n*.cab\n*.xar\n*.zst\n*.tzst\n\n# Packing-only formats\n*.iso\n*.tar\n\n# Package management formats\n*.dmg\n*.xpi\n*.gem\n*.egg\n*.deb\n*.rpm\n*.msi\n*.msm\n*.msp\n*.txz\n", #[cfg(feature = "global-backup")] Self::Backup => "*.bak\n*.gho\n*.ori\n*.orig\n*.tmp\n", #[cfg(feature = "global-bazaar")] Self::Bazaar => ".bzr/\n.bzrignore\n", #[cfg(feature = "global-bricx-cc")] Self::BricxCc => "# Bricx Command Center IDE\n# http://bricxcc.sourceforge.net\n*.bak\n*.sym\n", #[cfg(feature = "global-calabash")] Self::Calabash => "# Calabash / Cucumber\nrerun/\nreports/\nscreenshots/\nscreenshot*.png\ntest-servers/\n\n# bundler\n.bundle\nvendor\n", #[cfg(feature = "global-cloud9")] Self::Cloud9 => "# Cloud9 IDE - http://c9.io\n.c9revisions\n.c9\n", #[cfg(feature = "global-code-kit")] Self::CodeKit => "# General CodeKit files to ignore\nconfig.codekit\nconfig.codekit3\n/min\n", #[cfg(feature = "global-cursor")] Self::Cursor => ".cursorignore\n.cursorindexingignore\n", #[cfg(feature = "global-cvs")] Self::Cvs => "/CVS/*\n**/CVS/*\n.cvsignore\n*/.cvsignore\n", #[cfg(feature = "global-dart-editor")] Self::DartEditor => ".project\n.buildlog\n", #[cfg(feature = "global-diff")] Self::Diff => "*.patch\n*.diff\n", #[cfg(feature = "global-dreamweaver")] Self::Dreamweaver => "# DW Dreamweaver added files\n_notes\n_compareTemp\nconfigs/\ndwsync.xml\ndw_php_codehinting.config\n*.mno\n", #[cfg(feature = "global-dropbox")] Self::Dropbox => "# Dropbox settings and caches\n.dropbox\n.dropbox.attr\n.dropbox.cache\n", #[cfg(feature = "global-eclipse")] Self::Eclipse => ".metadata\nbin/\ntmp/\n*.tmp\n*.bak\n*.swp\n*~.nib\nlocal.properties\n.settings/\n.loadpath\n.recommenders\n\n# External tool builders\n.externalToolBuilders/\n\n# Locally stored \"Eclipse launch configurations\"\n*.launch\n\n# PyDev specific (Python IDE for Eclipse)\n*.pydevproject\n\n# CDT-specific (C/C++ Development Tooling)\n.cproject\n\n# CDT- autotools\n.autotools\n\n# Java annotation processor (APT)\n.factorypath\n\n# PDT-specific (PHP Development Tools)\n.buildpath\n\n# sbteclipse plugin\n.target\n\n# Tern plugin\n.tern-project\n\n# TeXlipse plugin\n.texlipse\n\n# STS (Spring Tool Suite)\n.springBeans\n\n# Code Recommenders\n.recommenders/\n\n# Annotation Processing\n.apt_generated/\n.apt_generated_test/\n\n# Scala IDE specific (Scala & Java development for Eclipse)\n.cache-main\n.scala_dependencies\n.worksheet\n\n# Uncomment this line if you wish to ignore the project description file.\n# Typically, this file would be tracked if it contains build/dependency configurations:\n#.project\n", #[cfg(feature = "global-eiffel-studio")] Self::EiffelStudio => "# The compilation directory\nEIFGENs\n", #[cfg(feature = "global-emacs")] Self::Emacs => "# -*- mode: gitignore; -*-\n*~\n\\#*\\#\n/.emacs.desktop\n/.emacs.desktop.lock\n*.elc\nauto-save-list\ntramp\n.\\#*\n\n# Org-mode\n.org-id-locations\n*_archive\n\n# flymake-mode\n*_flymake.*\n\n# eshell files\n/eshell/history\n/eshell/lastdir\n\n# elpa packages\n/elpa/\n\n# reftex files\n*.rel\n\n# AUCTeX auto folder\n/auto/\n\n# cask packages\n.cask/\ndist/\n\n# Flycheck\nflycheck_*.el\n\n# server auth directory\n/server/\n\n# projectiles files\n.projectile\n\n# directory configuration\n.dir-locals.el\n\n# network security\n/network-security.data\n\n", #[cfg(feature = "global-ensime")] Self::Ensime => "# Ensime specific\n.ensime\n.ensime_cache/\n.ensime_lucene/\n", #[cfg(feature = "global-espresso")] Self::Espresso => "*.esproj\n", #[cfg(feature = "global-flex-builder")] Self::FlexBuilder => "bin/\nbin-debug/\nbin-release/\n", #[cfg(feature = "global-gpg")] Self::Gpg => "secring.*\n\n", #[cfg(feature = "global-images")] Self::Images => "# JPEG\n*.jpg\n*.jpeg\n*.jpe\n*.jif\n*.jfif\n*.jfi\n\n# JPEG 2000\n*.jp2\n*.j2k\n*.jpf\n*.jpx\n*.jpm\n*.mj2\n\n# JPEG XR\n*.jxr\n*.hdp\n*.wdp\n\n# Graphics Interchange Format\n*.gif\n\n# RAW\n*.raw\n\n# Web P\n*.webp\n\n# Portable Network Graphics\n*.png\n\n# Animated Portable Network Graphics\n*.apng\n\n# Multiple-image Network Graphics\n*.mng\n\n# Tagged Image File Format\n*.tiff\n*.tif\n\n# Scalable Vector Graphics\n*.svg\n*.svgz\n\n# Portable Document Format\n*.pdf\n\n# X BitMap\n*.xbm\n\n# BMP\n*.bmp\n*.dib\n\n# ICO\n*.ico\n\n# 3D Images\n*.3dm\n*.max\n", #[cfg(feature = "global-j-developer")] Self::JDeveloper => "# default application storage directory used by the IDE Performance Cache feature\n.data/\n\n# used for ADF styles caching\ntemp/\n\n# default output directories\nclasses/\ndeploy/\njavadoc/\n\n# lock file, a part of Oracle Credential Store Framework\ncwallet.sso.lck", #[cfg(feature = "global-j-env")] Self::JEnv => "# JEnv local Java version configuration file\n.java-version\n\n# Used by previous versions of JEnv\n.jenv-version\n", #[cfg(feature = "global-jet-brains")] Self::JetBrains => "# Covers JetBrains IDEs: IntelliJ, GoLand, RubyMine, PhpStorm, AppCode, PyCharm, CLion, Android Studio, WebStorm and Rider\n# Reference: https://intellij-support.jetbrains.com/hc/en-us/articles/206544839\n\n# User-specific stuff\n.idea/**/workspace.xml\n.idea/**/tasks.xml\n.idea/**/usage.statistics.xml\n.idea/**/dictionaries\n.idea/**/shelf\n\n# AWS User-specific\n.idea/**/aws.xml\n\n# Generated files\n.idea/**/contentModel.xml\n\n# Sensitive or high-churn files\n.idea/**/dataSources/\n.idea/**/dataSources.ids\n.idea/**/dataSources.local.xml\n.idea/**/sqlDataSources.xml\n.idea/**/dynamic.xml\n.idea/**/uiDesigner.xml\n.idea/**/dbnavigator.xml\n\n# Gradle\n.idea/**/gradle.xml\n.idea/**/libraries\n\n# Gradle and Maven with auto-import\n# When using Gradle or Maven with auto-import, you should exclude module files,\n# since they will be recreated, and may cause churn.  Uncomment if using\n# auto-import.\n# .idea/artifacts\n# .idea/compiler.xml\n# .idea/jarRepositories.xml\n# .idea/modules.xml\n# .idea/*.iml\n# .idea/modules\n# *.iml\n# *.ipr\n\n# CMake\ncmake-build-*/\n\n# Mongo Explorer plugin\n.idea/**/mongoSettings.xml\n\n# File-based project format\n*.iws\n\n# IntelliJ\nout/\n\n# mpeltonen/sbt-idea plugin\n.idea_modules/\n\n# JIRA plugin\natlassian-ide-plugin.xml\n\n# Cursive Clojure plugin\n.idea/replstate.xml\n\n# SonarLint plugin\n.idea/sonarlint/\n.idea/sonarlint.xml # see https://community.sonarsource.com/t/is-the-file-idea-idea-idea-sonarlint-xml-intended-to-be-under-source-control/121119\n\n# Crashlytics plugin (for Android Studio and IntelliJ)\ncom_crashlytics_export_strings.xml\ncrashlytics.properties\ncrashlytics-build.properties\nfabric.properties\n\n# Editor-based Rest Client\n.idea/httpRequests\n\n# Android studio 3.1+ serialized cache file\n.idea/caches/build_file_checksums.ser\n", #[cfg(feature = "global-k-develop4")] Self::KDevelop4 => "*.kdev4\n.kdev4/\n", #[cfg(feature = "global-kate")] Self::Kate => "# Swap Files #\n.*.kate-swp\n.swp.*\n", #[cfg(feature = "global-lazarus")] Self::Lazarus => "# Lazarus compiler-generated binaries (safe to delete)\n*.exe\n*.dll\n*.so\n*.dylib\n*.lrs\n*.res\n*.compiled\n*.dbg\n*.ppu\n*.o\n*.or\n*.a\n\n# Lazarus autogenerated files (duplicated info)\n*.rst\n*.rsj\n*.lrt\n\n# Lazarus local files (user-specific info)\n*.lps\n\n# Lazarus backups and unit output folders.\n# These can be changed by user in Lazarus/project options.\nbackup/\n*.bak\nlib/\n\n# Application bundle for Mac OS\n*.app/\n", #[cfg(feature = "global-libre-office")] Self::LibreOffice => "# LibreOffice locks\n.~lock.*#\n", #[cfg(feature = "global-linux")] Self::Linux => "*~\n\n# temporary files which can be created if a process still has a handle open of a deleted file\n.fuse_hidden*\n\n# Metadata left by Dolphin file manager, which comes with KDE Plasma\n.directory\n\n# Linux trash folder which might appear on any partition or disk\n.Trash-*\n\n# .nfs files are created when an open file is removed but is still being accessed\n.nfs*\n\n# Log files created by default by the nohup command\nnohup.out\n", #[cfg(feature = "global-ly-x")] Self::LyX => "# Ignore LyX backup and autosave files\n# http://www.lyx.org/\n*.lyx~\n*.lyx#\n", #[cfg(feature = "global-mac-os")] Self::MacOs => "# General\n.DS_Store\n.AppleDouble\n.LSOverride\nIcon[\r]\n\n# Thumbnails\n._*\n\n# Files that might appear in the root of a volume\n.DocumentRevisions-V100\n.fseventsd\n.Spotlight-V100\n.TemporaryItems\n.Trashes\n.VolumeIcon.icns\n.com.apple.timemachine.donotpresent\n\n# Directories potentially created on remote AFP share\n.AppleDB\n.AppleDesktop\nNetwork Trash Folder\nTemporary Items\n.apdisk\n", #[cfg(feature = "global-matlab")] Self::Matlab => "# Windows default autosave extension\n*.asv\n\n# OSX / *nix default autosave extension\n*.m~\n\n# Compiled MEX binaries (all platforms)\n*.mex*\n\n# Packaged app and toolbox files\n*.mlappinstall\n*.mltbx\n\n# Generated helpsearch folders\nhelpsearch*/\n\n# Simulink code generation folders\nslprj/\nsccprj/\n\n# Matlab code generation folders\ncodegen/\n\n# Simulink autosave extension\n*.autosave\n\n# Simulink cache files\n*.slxc\n\n# Octave session info\noctave-workspace\n", #[cfg(feature = "global-mercurial")] Self::Mercurial => ".hg/\n.hgignore\n.hgsigs\n.hgsub\n.hgsubstate\n.hgtags\n", #[cfg(feature = "global-metals")] Self::Metals => "# Metals (Scala Language Server)\n# Reference: https://scalameta.org/metals/docs/editors/vscode#files-and-directories-to-include-in-your-gitignore\n.metals/\n.bloop/\n.ammonite/\nmetals.sbt\n", #[cfg(feature = "global-microsoft-office")] Self::MicrosoftOffice => "*.tmp\n\n# Word temporary\n~$*.doc*\n\n# Word Auto Backup File\nBackup of *.doc*\n\n# Excel temporary\n~$*.xls*\n\n# Excel Backup File\n*.xlk\n\n# PowerPoint temporary\n~$*.ppt*\n\n# Visio autosave temporary files\n*.~vsd*\n", #[cfg(feature = "global-model-sim")] Self::ModelSim => "# ignore ModelSim generated files and directories (temp files and so on)\n[_@]*\n\n# ignore compilation output of ModelSim\n*.mti\n*.dat\n*.dbs\n*.psm\n*.bak\n*.cmp\n*.jpg\n*.html\n*.bsf\n\n# ignore simulation output of ModelSim\nwlf*\n*.wlf\n*.vstf\n*.ucdb\ncov*/\ntranscript*\nsc_dpiheader.h\nvsim.dbg\n", #[cfg(feature = "global-momentics")] Self::Momentics => "# Built files\nx86/\narm/\narm-p/\ntranslations/*.qm\n\n# IDE settings\n.settings/\n", #[cfg(feature = "global-mono-develop")] Self::MonoDevelop => "#User Specific\n*.userprefs\n*.usertasks\n\n#Mono Project Files\n*.pidb\n*.resources\ntest-results/\n", #[cfg(feature = "global-net-beans")] Self::NetBeans => "**/nbproject/private/\n**/nbproject/Makefile-*.mk\n**/nbproject/Package-*.bash\nbuild/\nnbbuild/\ndist/\nnbdist/\n.nb-gradle/\n", #[cfg(feature = "global-ninja")] Self::Ninja => ".ninja_deps\n.ninja_log\n", #[cfg(feature = "global-notepad-pp")] Self::NotepadPp => "# Notepad++ backups #\r\n*.bak\r\n", #[cfg(feature = "global-octave")] Self::Octave => "# Windows default autosave extension\n*.asv\n\n# OSX / *nix default autosave extension\n*.m~\n\n# Compiled MEX binaries (all platforms)\n*.mex*\n\n# Packaged app and toolbox files\n*.mlappinstall\n*.mltbx\n\n# Generated helpsearch folders\nhelpsearch*/\n\n# Simulink code generation folders\nslprj/\nsccprj/\n\n# Matlab code generation folders\ncodegen/\n\n# Simulink autosave extension\n*.autosave\n\n# Simulink cache files\n*.slxc\n\n# Octave session info\noctave-workspace\n", #[cfg(feature = "global-otto")] Self::Otto => ".otto/\n", #[cfg(feature = "global-p-so-c-creator")] Self::PSoCCreator => "# Project Settings\n*.cywrk.*\n*.cyprj.*\n\n# Generated Assets and Resources\nDebug/\nRelease/\nExport/\n*/codegentemp\n*/Generated_Source\n*_datasheet.pdf\n*_timing.html\n*.cycdx\n*.cyfit\n*.rpt\n*.svd\n*.log\n*.zip\n", #[cfg(feature = "global-patch")] Self::Patch => "*.orig\n*.rej\n", #[cfg(feature = "global-platform-io")] Self::PlatformIo => ".pio\n.pioenvs\n.piolibdeps\n.vscode/.browse.c_cpp.db*\n.vscode/c_cpp_properties.json\n.vscode/launch.json\n", #[cfg(feature = "global-pu-tty")] Self::PuTty => "# Private key\n*.ppk\n", #[cfg(feature = "global-redcar")] Self::Redcar => ".redcar\n", #[cfg(feature = "global-redis")] Self::Redis => "# Ignore redis binary dump (dump.rdb) files\n\n*.rdb\n", #[cfg(feature = "global-sbt")] Self::Sbt => "# Simple Build Tool\n# http://www.scala-sbt.org/release/docs/Getting-Started/Directories.html#configuring-version-control\n\ndist/*\ntarget/\nlib_managed/\nsrc_managed/\nproject/boot/\nproject/plugins/project/\n.history\n.cache\n.lib/\n", #[cfg(feature = "global-slick-edit")] Self::SlickEdit => "# SlickEdit workspace and project files are ignored by default because\n# typically they are considered to be developer-specific and not part of a\n# project.\n*.vpw\n*.vpj\n\n# SlickEdit workspace history and tag files always contain user-specific\n# data so they should not be stored in a repository.\n*.vpwhistu\n*.vpwhist\n*.vtg\n", #[cfg(feature = "global-stata")] Self::Stata => "# .gitignore file for git projects containing Stata files\n# Commercial statistical software: http://www.stata.com\n\n# Stata dataset and output files\n*.dta\n*.gph\n*.log\n*.smcl\n*.stpr\n*.stsem\n\n# Graphic export files from Stata\n# Stata command graph export: http://www.stata.com/manuals14/g-2graphexport.pdf\n#\n# You may add graphic export files to your .gitignore. However you should be\n# aware that this will exclude all image files from this main directory\n# and subdirectories.\n# *.ps\n# *.eps\n# *.wmf\n# *.emf\n# *.pdf\n# *.png\n# *.tif\n", #[cfg(feature = "global-sublime-text")] Self::SublimeText => "# Cache files for Sublime Text\n*.tmlanguage.cache\n*.tmPreferences.cache\n*.stTheme.cache\n\n# Workspace files are user-specific\n*.sublime-workspace\n\n# Project files should be checked into the repository, unless a significant\n# proportion of contributors will probably not be using Sublime Text\n# *.sublime-project\n\n# SFTP configuration file\nsftp-config.json\nsftp-config-alt*.json\n\n# Package control specific files\nPackage Control.last-run\nPackage Control.ca-list\nPackage Control.ca-bundle\nPackage Control.system-ca-bundle\nPackage Control.cache/\nPackage Control.ca-certs/\nPackage Control.merged-ca-bundle\nPackage Control.user-ca-bundle\noscrypto-ca-bundle.crt\nbh_unicode_properties.cache\n\n# Sublime-github package stores a github token in this file\n# https://packagecontrol.io/packages/sublime-github\nGitHub.sublime-settings\n", #[cfg(feature = "global-svn")] Self::Svn => ".svn/\n", #[cfg(feature = "global-syncthing")] Self::Syncthing => "# Syncthing caches\n.stversions\n", #[cfg(feature = "global-synopsys-vcs")] Self::SynopsysVcs => "# Waveform formats\n*.vcd\n*.vpd\n*.evcd\n*.fsdb\n\n# Default name of the simulation executable.  A different name can be\n# specified with this switch (the associated daidir database name is\n# also taken from here):  -o <path>/<filename>\nsimv\n\n# Generated for Verilog and VHDL top configs\nsimv.daidir/\nsimv.db.dir/\n\n# Infrastructure necessary to co-simulate SystemC models with\n# Verilog/VHDL models.  An alternate directory may be specified with this\n# switch:  -Mdir=<directory_path>\ncsrc/\n\n# Log file - the following switch allows to specify the file that will be\n# used to write all messages from simulation:  -l <filename>\n*.log\n\n# Coverage results (generated with urg) and database location.  The\n# following switch can also be used:  urg -dir <coverage_directory>.vdb\nsimv.vdb/\nurgReport/\n\n# DVE and UCLI related files.\nDVEfiles/\nucli.key\n\n# When the design is elaborated for DirectC, the following file is created\n# with declarations for C/C++ functions.\nvc_hdrs.h\n", #[cfg(feature = "global-tags")] Self::Tags => "# Ignore tags created by etags, ctags, gtags (GNU global) and cscope\nTAGS\n.TAGS\n!TAGS/\ntags\n.tags\n!tags/\ngtags.files\nGTAGS\nGRTAGS\nGPATH\nGSYMS\ncscope.files\ncscope.out\ncscope.in.out\ncscope.po.out\n\n", #[cfg(feature = "global-text-mate")] Self::TextMate => "*.tmproj\n*.tmproject\ntmtags\n", #[cfg(feature = "global-tortoise-git")] Self::TortoiseGit => "# Project-level settings\n/.tgitconfig\n", #[cfg(feature = "global-vagrant")] Self::Vagrant => "# General\n.vagrant/\n\n# Log files (if you are creating logs in debug mode, uncomment this)\n# *.log\n", #[cfg(feature = "global-vim")] Self::Vim => "# Swap\n[._]*.s[a-v][a-z]\n!*.svg  # comment out if you don't need vector files\n[._]*.sw[a-p]\n[._]s[a-rt-v][a-z]\n[._]ss[a-gi-z]\n[._]sw[a-p]\n\n# Session\nSession.vim\nSessionx.vim\n\n# Temporary\n.netrwhist\n*~\n# Auto-generated tag files\ntags\n# Persistent undo\n[._]*.un~\n", #[cfg(feature = "global-virtual-env")] Self::VirtualEnv => "# Virtualenv\n# http://iamzed.com/2009/05/07/a-primer-on-virtualenv/\n.Python\n[Bb]in\n[Ii]nclude\n[Ll]ib\n[Ll]ib64\n[Ll]ocal\n[Ss]cripts\npyvenv.cfg\n.venv\npip-selfcheck.json\n", #[cfg(feature = "global-virtuoso")] Self::Virtuoso => "# Gitignore for Cadence Virtuoso\n################################################################\n\n# Log files\n*.log\npanic*.log.*\n\n# OpenAccess database lock files\n*.cdslck*\n\n# Run directories for layout vs. schematic and design rule check\nlvsRunDir/*\ndrcRunDir/*\n\n# Abstract generation tool\nabstract.log*\nabstract.record*\n\n", #[cfg(feature = "global-visual-studio-code")] Self::VisualStudioCode => ".vscode/*\n!.vscode/settings.json\n!.vscode/tasks.json\n!.vscode/launch.json\n!.vscode/extensions.json\n!.vscode/*.code-snippets\n\n# Local History for Visual Studio Code\n.history/\n\n# Built Visual Studio Code Extensions\n*.vsix\n", #[cfg(feature = "global-web-methods")] Self::WebMethods => "**/IntegrationServer/datastore/\n**/IntegrationServer/db/\n**/IntegrationServer/DocumentStore/\n**/IntegrationServer/lib/\n**/IntegrationServer/logs/\n**/IntegrationServer/replicate/\n**/IntegrationServer/sdk/\n**/IntegrationServer/support/\n**/IntegrationServer/update/\n**/IntegrationServer/userFtpRoot/\n**/IntegrationServer/web/\n**/IntegrationServer/WmRepository4/\n**/IntegrationServer/XAStore/\n**/IntegrationServer/packages/Wm*/\n", #[cfg(feature = "global-windows")] Self::Windows => "# Windows thumbnail cache files\nThumbs.db\nThumbs.db:encryptable\nehthumbs.db\nehthumbs_vista.db\n\n# Dump file\n*.stackdump\n\n# Folder config file\n[Dd]esktop.ini\n\n# Recycle Bin used on file shares\n$RECYCLE.BIN/\n\n# Windows Installer files\n*.cab\n*.msi\n*.msix\n*.msm\n*.msp\n\n# Windows shortcuts\n*.lnk\n", #[cfg(feature = "global-xcode")] Self::Xcode => "## User settings\nxcuserdata/\n", #[cfg(feature = "global-xilinx-ise")] Self::XilinxIse => "# intermediate build files\n*.bgn\n*.bit\n*.bld\n*.cmd_log\n*.drc\n*.ll\n*.lso\n*.msd\n*.msk\n*.ncd\n*.ngc\n*.ngd\n*.ngr\n*.pad\n*.par\n*.pcf\n*.prj\n*.ptwx\n*.rbb\n*.rbd\n*.stx\n*.syr\n*.twr\n*.twx\n*.unroutes\n*.ut\n*.xpi\n*.xst\n*_bitgen.xwbt\n*_envsettings.html\n*_map.map\n*_map.mrp\n*_map.ngm\n*_map.xrpt\n*_ngdbuild.xrpt\n*_pad.csv\n*_pad.txt\n*_par.xrpt\n*_summary.html\n*_summary.xml\n*_usage.xml\n*_xst.xrpt\n\n# iMPACT generated files\n_impactbatch.log\nimpact.xsl\nimpact_impact.xwbt\nise_impact.cmd\nwebtalk_impact.xml\n\n# Core Generator generated files\nxaw2verilog.log\n\n# project-wide generated files\n*.gise\npar_usage_statistics.html\nusage_statistics_webtalk.html\nwebtalk.log\nwebtalk_pn.xml\n\n# generated folders\niseconfig/\nxlnx_auto_0_xdb/\nxst/\n_ngo/\n_xmsgs/\n" }
	}

	fn file_name(self) -> &'static str {
		match self {
			#[cfg(feature = "global-al")]
			Self::Al => "AL.gitignore",
			#[cfg(feature = "global-anjuta")]
			Self::Anjuta => "Anjuta.gitignore",
			#[cfg(feature = "global-ansible")]
			Self::Ansible => "Ansible.gitignore",
			#[cfg(feature = "global-archives")]
			Self::Archives => "Archives.gitignore",
			#[cfg(feature = "global-backup")]
			Self::Backup => "Backup.gitignore",
			#[cfg(feature = "global-bazaar")]
			Self::Bazaar => "Bazaar.gitignore",
			#[cfg(feature = "global-bricx-cc")]
			Self::BricxCc => "BricxCC.gitignore",
			#[cfg(feature = "global-calabash")]
			Self::Calabash => "Calabash.gitignore",
			#[cfg(feature = "global-cloud9")]
			Self::Cloud9 => "Cloud9.gitignore",
			#[cfg(feature = "global-code-kit")]
			Self::CodeKit => "CodeKit.gitignore",
			#[cfg(feature = "global-cursor")]
			Self::Cursor => "Cursor.gitignore",
			#[cfg(feature = "global-cvs")]
			Self::Cvs => "CVS.gitignore",
			#[cfg(feature = "global-dart-editor")]
			Self::DartEditor => "DartEditor.gitignore",
			#[cfg(feature = "global-diff")]
			Self::Diff => "Diff.gitignore",
			#[cfg(feature = "global-dreamweaver")]
			Self::Dreamweaver => "Dreamweaver.gitignore",
			#[cfg(feature = "global-dropbox")]
			Self::Dropbox => "Dropbox.gitignore",
			#[cfg(feature = "global-eclipse")]
			Self::Eclipse => "Eclipse.gitignore",
			#[cfg(feature = "global-eiffel-studio")]
			Self::EiffelStudio => "EiffelStudio.gitignore",
			#[cfg(feature = "global-emacs")]
			Self::Emacs => "Emacs.gitignore",
			#[cfg(feature = "global-ensime")]
			Self::Ensime => "Ensime.gitignore",
			#[cfg(feature = "global-espresso")]
			Self::Espresso => "Espresso.gitignore",
			#[cfg(feature = "global-flex-builder")]
			Self::FlexBuilder => "FlexBuilder.gitignore",
			#[cfg(feature = "global-gpg")]
			Self::Gpg => "GPG.gitignore",
			#[cfg(feature = "global-images")]
			Self::Images => "Images.gitignore",
			#[cfg(feature = "global-j-developer")]
			Self::JDeveloper => "JDeveloper.gitignore",
			#[cfg(feature = "global-j-env")]
			Self::JEnv => "JEnv.gitignore",
			#[cfg(feature = "global-jet-brains")]
			Self::JetBrains => "JetBrains.gitignore",
			#[cfg(feature = "global-k-develop4")]
			Self::KDevelop4 => "KDevelop4.gitignore",
			#[cfg(feature = "global-kate")]
			Self::Kate => "Kate.gitignore",
			#[cfg(feature = "global-lazarus")]
			Self::Lazarus => "Lazarus.gitignore",
			#[cfg(feature = "global-libre-office")]
			Self::LibreOffice => "LibreOffice.gitignore",
			#[cfg(feature = "global-linux")]
			Self::Linux => "Linux.gitignore",
			#[cfg(feature = "global-ly-x")]
			Self::LyX => "LyX.gitignore",
			#[cfg(feature = "global-mac-os")]
			Self::MacOs => "macOS.gitignore",
			#[cfg(feature = "global-matlab")]
			Self::Matlab => "MATLAB.gitignore",
			#[cfg(feature = "global-mercurial")]
			Self::Mercurial => "Mercurial.gitignore",
			#[cfg(feature = "global-metals")]
			Self::Metals => "Metals.gitignore",
			#[cfg(feature = "global-microsoft-office")]
			Self::MicrosoftOffice => "MicrosoftOffice.gitignore",
			#[cfg(feature = "global-model-sim")]
			Self::ModelSim => "ModelSim.gitignore",
			#[cfg(feature = "global-momentics")]
			Self::Momentics => "Momentics.gitignore",
			#[cfg(feature = "global-mono-develop")]
			Self::MonoDevelop => "MonoDevelop.gitignore",
			#[cfg(feature = "global-net-beans")]
			Self::NetBeans => "NetBeans.gitignore",
			#[cfg(feature = "global-ninja")]
			Self::Ninja => "Ninja.gitignore",
			#[cfg(feature = "global-notepad-pp")]
			Self::NotepadPp => "NotepadPP.gitignore",
			#[cfg(feature = "global-octave")]
			Self::Octave => "Octave.gitignore",
			#[cfg(feature = "global-otto")]
			Self::Otto => "Otto.gitignore",
			#[cfg(feature = "global-p-so-c-creator")]
			Self::PSoCCreator => "PSoCCreator.gitignore",
			#[cfg(feature = "global-patch")]
			Self::Patch => "Patch.gitignore",
			#[cfg(feature = "global-platform-io")]
			Self::PlatformIo => "PlatformIO.gitignore",
			#[cfg(feature = "global-pu-tty")]
			Self::PuTty => "PuTTY.gitignore",
			#[cfg(feature = "global-redcar")]
			Self::Redcar => "Redcar.gitignore",
			#[cfg(feature = "global-redis")]
			Self::Redis => "Redis.gitignore",
			#[cfg(feature = "global-sbt")]
			Self::Sbt => "SBT.gitignore",
			#[cfg(feature = "global-slick-edit")]
			Self::SlickEdit => "SlickEdit.gitignore",
			#[cfg(feature = "global-stata")]
			Self::Stata => "Stata.gitignore",
			#[cfg(feature = "global-sublime-text")]
			Self::SublimeText => "SublimeText.gitignore",
			#[cfg(feature = "global-svn")]
			Self::Svn => "SVN.gitignore",
			#[cfg(feature = "global-syncthing")]
			Self::Syncthing => "Syncthing.gitignore",
			#[cfg(feature = "global-synopsys-vcs")]
			Self::SynopsysVcs => "SynopsysVCS.gitignore",
			#[cfg(feature = "global-tags")]
			Self::Tags => "Tags.gitignore",
			#[cfg(feature = "global-text-mate")]
			Self::TextMate => "TextMate.gitignore",
			#[cfg(feature = "global-tortoise-git")]
			Self::TortoiseGit => "TortoiseGit.gitignore",
			#[cfg(feature = "global-vagrant")]
			Self::Vagrant => "Vagrant.gitignore",
			#[cfg(feature = "global-vim")]
			Self::Vim => "Vim.gitignore",
			#[cfg(feature = "global-virtual-env")]
			Self::VirtualEnv => "VirtualEnv.gitignore",
			#[cfg(feature = "global-virtuoso")]
			Self::Virtuoso => "Virtuoso.gitignore",
			#[cfg(feature = "global-visual-studio-code")]
			Self::VisualStudioCode => "VisualStudioCode.gitignore",
			#[cfg(feature = "global-web-methods")]
			Self::WebMethods => "WebMethods.gitignore",
			#[cfg(feature = "global-windows")]
			Self::Windows => "Windows.gitignore",
			#[cfg(feature = "global-xcode")]
			Self::Xcode => "Xcode.gitignore",
			#[cfg(feature = "global-xilinx-ise")]
			Self::XilinxIse => "XilinxISE.gitignore",
		}
	}

	fn file_path(self) -> &'static str {
		match self {
			#[cfg(feature = "global-al")]
			Self::Al => "Global/AL.gitignore",
			#[cfg(feature = "global-anjuta")]
			Self::Anjuta => "Global/Anjuta.gitignore",
			#[cfg(feature = "global-ansible")]
			Self::Ansible => "Global/Ansible.gitignore",
			#[cfg(feature = "global-archives")]
			Self::Archives => "Global/Archives.gitignore",
			#[cfg(feature = "global-backup")]
			Self::Backup => "Global/Backup.gitignore",
			#[cfg(feature = "global-bazaar")]
			Self::Bazaar => "Global/Bazaar.gitignore",
			#[cfg(feature = "global-bricx-cc")]
			Self::BricxCc => "Global/BricxCC.gitignore",
			#[cfg(feature = "global-calabash")]
			Self::Calabash => "Global/Calabash.gitignore",
			#[cfg(feature = "global-cloud9")]
			Self::Cloud9 => "Global/Cloud9.gitignore",
			#[cfg(feature = "global-code-kit")]
			Self::CodeKit => "Global/CodeKit.gitignore",
			#[cfg(feature = "global-cursor")]
			Self::Cursor => "Global/Cursor.gitignore",
			#[cfg(feature = "global-cvs")]
			Self::Cvs => "Global/CVS.gitignore",
			#[cfg(feature = "global-dart-editor")]
			Self::DartEditor => "Global/DartEditor.gitignore",
			#[cfg(feature = "global-diff")]
			Self::Diff => "Global/Diff.gitignore",
			#[cfg(feature = "global-dreamweaver")]
			Self::Dreamweaver => "Global/Dreamweaver.gitignore",
			#[cfg(feature = "global-dropbox")]
			Self::Dropbox => "Global/Dropbox.gitignore",
			#[cfg(feature = "global-eclipse")]
			Self::Eclipse => "Global/Eclipse.gitignore",
			#[cfg(feature = "global-eiffel-studio")]
			Self::EiffelStudio => "Global/EiffelStudio.gitignore",
			#[cfg(feature = "global-emacs")]
			Self::Emacs => "Global/Emacs.gitignore",
			#[cfg(feature = "global-ensime")]
			Self::Ensime => "Global/Ensime.gitignore",
			#[cfg(feature = "global-espresso")]
			Self::Espresso => "Global/Espresso.gitignore",
			#[cfg(feature = "global-flex-builder")]
			Self::FlexBuilder => "Global/FlexBuilder.gitignore",
			#[cfg(feature = "global-gpg")]
			Self::Gpg => "Global/GPG.gitignore",
			#[cfg(feature = "global-images")]
			Self::Images => "Global/Images.gitignore",
			#[cfg(feature = "global-j-developer")]
			Self::JDeveloper => "Global/JDeveloper.gitignore",
			#[cfg(feature = "global-j-env")]
			Self::JEnv => "Global/JEnv.gitignore",
			#[cfg(feature = "global-jet-brains")]
			Self::JetBrains => "Global/JetBrains.gitignore",
			#[cfg(feature = "global-k-develop4")]
			Self::KDevelop4 => "Global/KDevelop4.gitignore",
			#[cfg(feature = "global-kate")]
			Self::Kate => "Global/Kate.gitignore",
			#[cfg(feature = "global-lazarus")]
			Self::Lazarus => "Global/Lazarus.gitignore",
			#[cfg(feature = "global-libre-office")]
			Self::LibreOffice => "Global/LibreOffice.gitignore",
			#[cfg(feature = "global-linux")]
			Self::Linux => "Global/Linux.gitignore",
			#[cfg(feature = "global-ly-x")]
			Self::LyX => "Global/LyX.gitignore",
			#[cfg(feature = "global-mac-os")]
			Self::MacOs => "Global/macOS.gitignore",
			#[cfg(feature = "global-matlab")]
			Self::Matlab => "Global/MATLAB.gitignore",
			#[cfg(feature = "global-mercurial")]
			Self::Mercurial => "Global/Mercurial.gitignore",
			#[cfg(feature = "global-metals")]
			Self::Metals => "Global/Metals.gitignore",
			#[cfg(feature = "global-microsoft-office")]
			Self::MicrosoftOffice => "Global/MicrosoftOffice.gitignore",
			#[cfg(feature = "global-model-sim")]
			Self::ModelSim => "Global/ModelSim.gitignore",
			#[cfg(feature = "global-momentics")]
			Self::Momentics => "Global/Momentics.gitignore",
			#[cfg(feature = "global-mono-develop")]
			Self::MonoDevelop => "Global/MonoDevelop.gitignore",
			#[cfg(feature = "global-net-beans")]
			Self::NetBeans => "Global/NetBeans.gitignore",
			#[cfg(feature = "global-ninja")]
			Self::Ninja => "Global/Ninja.gitignore",
			#[cfg(feature = "global-notepad-pp")]
			Self::NotepadPp => "Global/NotepadPP.gitignore",
			#[cfg(feature = "global-octave")]
			Self::Octave => "Global/Octave.gitignore",
			#[cfg(feature = "global-otto")]
			Self::Otto => "Global/Otto.gitignore",
			#[cfg(feature = "global-p-so-c-creator")]
			Self::PSoCCreator => "Global/PSoCCreator.gitignore",
			#[cfg(feature = "global-patch")]
			Self::Patch => "Global/Patch.gitignore",
			#[cfg(feature = "global-platform-io")]
			Self::PlatformIo => "Global/PlatformIO.gitignore",
			#[cfg(feature = "global-pu-tty")]
			Self::PuTty => "Global/PuTTY.gitignore",
			#[cfg(feature = "global-redcar")]
			Self::Redcar => "Global/Redcar.gitignore",
			#[cfg(feature = "global-redis")]
			Self::Redis => "Global/Redis.gitignore",
			#[cfg(feature = "global-sbt")]
			Self::Sbt => "Global/SBT.gitignore",
			#[cfg(feature = "global-slick-edit")]
			Self::SlickEdit => "Global/SlickEdit.gitignore",
			#[cfg(feature = "global-stata")]
			Self::Stata => "Global/Stata.gitignore",
			#[cfg(feature = "global-sublime-text")]
			Self::SublimeText => "Global/SublimeText.gitignore",
			#[cfg(feature = "global-svn")]
			Self::Svn => "Global/SVN.gitignore",
			#[cfg(feature = "global-syncthing")]
			Self::Syncthing => "Global/Syncthing.gitignore",
			#[cfg(feature = "global-synopsys-vcs")]
			Self::SynopsysVcs => "Global/SynopsysVCS.gitignore",
			#[cfg(feature = "global-tags")]
			Self::Tags => "Global/Tags.gitignore",
			#[cfg(feature = "global-text-mate")]
			Self::TextMate => "Global/TextMate.gitignore",
			#[cfg(feature = "global-tortoise-git")]
			Self::TortoiseGit => "Global/TortoiseGit.gitignore",
			#[cfg(feature = "global-vagrant")]
			Self::Vagrant => "Global/Vagrant.gitignore",
			#[cfg(feature = "global-vim")]
			Self::Vim => "Global/Vim.gitignore",
			#[cfg(feature = "global-virtual-env")]
			Self::VirtualEnv => "Global/VirtualEnv.gitignore",
			#[cfg(feature = "global-virtuoso")]
			Self::Virtuoso => "Global/Virtuoso.gitignore",
			#[cfg(feature = "global-visual-studio-code")]
			Self::VisualStudioCode => "Global/VisualStudioCode.gitignore",
			#[cfg(feature = "global-web-methods")]
			Self::WebMethods => "Global/WebMethods.gitignore",
			#[cfg(feature = "global-windows")]
			Self::Windows => "Global/Windows.gitignore",
			#[cfg(feature = "global-xcode")]
			Self::Xcode => "Global/Xcode.gitignore",
			#[cfg(feature = "global-xilinx-ise")]
			Self::XilinxIse => "Global/XilinxISE.gitignore",
		}
	}

	#[cfg(feature = "std")]
	fn list() -> Vec<&'static str> {
		#[allow(unused_mut)]
		let mut list = Vec::with_capacity(71);
		#[cfg(feature = "global-al")]
		list.push("Al");
		#[cfg(feature = "global-anjuta")]
		list.push("Anjuta");
		#[cfg(feature = "global-ansible")]
		list.push("Ansible");
		#[cfg(feature = "global-archives")]
		list.push("Archives");
		#[cfg(feature = "global-backup")]
		list.push("Backup");
		#[cfg(feature = "global-bazaar")]
		list.push("Bazaar");
		#[cfg(feature = "global-bricx-cc")]
		list.push("BricxCc");
		#[cfg(feature = "global-calabash")]
		list.push("Calabash");
		#[cfg(feature = "global-cloud9")]
		list.push("Cloud9");
		#[cfg(feature = "global-code-kit")]
		list.push("CodeKit");
		#[cfg(feature = "global-cursor")]
		list.push("Cursor");
		#[cfg(feature = "global-cvs")]
		list.push("Cvs");
		#[cfg(feature = "global-dart-editor")]
		list.push("DartEditor");
		#[cfg(feature = "global-diff")]
		list.push("Diff");
		#[cfg(feature = "global-dreamweaver")]
		list.push("Dreamweaver");
		#[cfg(feature = "global-dropbox")]
		list.push("Dropbox");
		#[cfg(feature = "global-eclipse")]
		list.push("Eclipse");
		#[cfg(feature = "global-eiffel-studio")]
		list.push("EiffelStudio");
		#[cfg(feature = "global-emacs")]
		list.push("Emacs");
		#[cfg(feature = "global-ensime")]
		list.push("Ensime");
		#[cfg(feature = "global-espresso")]
		list.push("Espresso");
		#[cfg(feature = "global-flex-builder")]
		list.push("FlexBuilder");
		#[cfg(feature = "global-gpg")]
		list.push("Gpg");
		#[cfg(feature = "global-images")]
		list.push("Images");
		#[cfg(feature = "global-j-developer")]
		list.push("JDeveloper");
		#[cfg(feature = "global-j-env")]
		list.push("JEnv");
		#[cfg(feature = "global-jet-brains")]
		list.push("JetBrains");
		#[cfg(feature = "global-k-develop4")]
		list.push("KDevelop4");
		#[cfg(feature = "global-kate")]
		list.push("Kate");
		#[cfg(feature = "global-lazarus")]
		list.push("Lazarus");
		#[cfg(feature = "global-libre-office")]
		list.push("LibreOffice");
		#[cfg(feature = "global-linux")]
		list.push("Linux");
		#[cfg(feature = "global-ly-x")]
		list.push("LyX");
		#[cfg(feature = "global-mac-os")]
		list.push("MacOs");
		#[cfg(feature = "global-matlab")]
		list.push("Matlab");
		#[cfg(feature = "global-mercurial")]
		list.push("Mercurial");
		#[cfg(feature = "global-metals")]
		list.push("Metals");
		#[cfg(feature = "global-microsoft-office")]
		list.push("MicrosoftOffice");
		#[cfg(feature = "global-model-sim")]
		list.push("ModelSim");
		#[cfg(feature = "global-momentics")]
		list.push("Momentics");
		#[cfg(feature = "global-mono-develop")]
		list.push("MonoDevelop");
		#[cfg(feature = "global-net-beans")]
		list.push("NetBeans");
		#[cfg(feature = "global-ninja")]
		list.push("Ninja");
		#[cfg(feature = "global-notepad-pp")]
		list.push("NotepadPp");
		#[cfg(feature = "global-octave")]
		list.push("Octave");
		#[cfg(feature = "global-otto")]
		list.push("Otto");
		#[cfg(feature = "global-p-so-c-creator")]
		list.push("PSoCCreator");
		#[cfg(feature = "global-patch")]
		list.push("Patch");
		#[cfg(feature = "global-platform-io")]
		list.push("PlatformIo");
		#[cfg(feature = "global-pu-tty")]
		list.push("PuTty");
		#[cfg(feature = "global-redcar")]
		list.push("Redcar");
		#[cfg(feature = "global-redis")]
		list.push("Redis");
		#[cfg(feature = "global-sbt")]
		list.push("Sbt");
		#[cfg(feature = "global-slick-edit")]
		list.push("SlickEdit");
		#[cfg(feature = "global-stata")]
		list.push("Stata");
		#[cfg(feature = "global-sublime-text")]
		list.push("SublimeText");
		#[cfg(feature = "global-svn")]
		list.push("Svn");
		#[cfg(feature = "global-syncthing")]
		list.push("Syncthing");
		#[cfg(feature = "global-synopsys-vcs")]
		list.push("SynopsysVcs");
		#[cfg(feature = "global-tags")]
		list.push("Tags");
		#[cfg(feature = "global-text-mate")]
		list.push("TextMate");
		#[cfg(feature = "global-tortoise-git")]
		list.push("TortoiseGit");
		#[cfg(feature = "global-vagrant")]
		list.push("Vagrant");
		#[cfg(feature = "global-vim")]
		list.push("Vim");
		#[cfg(feature = "global-virtual-env")]
		list.push("VirtualEnv");
		#[cfg(feature = "global-virtuoso")]
		list.push("Virtuoso");
		#[cfg(feature = "global-visual-studio-code")]
		list.push("VisualStudioCode");
		#[cfg(feature = "global-web-methods")]
		list.push("WebMethods");
		#[cfg(feature = "global-windows")]
		list.push("Windows");
		#[cfg(feature = "global-xcode")]
		list.push("Xcode");
		#[cfg(feature = "global-xilinx-ise")]
		list.push("XilinxIse");
		list
	}
}

#[cfg(all(feature = "std", not(feature = "no-contents")))]
impl std::fmt::Display for Global {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.contents())
	}
}
