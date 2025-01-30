
use crate::GitIgnore;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Community {
	#[cfg(feature = "community-alteryx")]
	Alteryx,
	#[cfg(feature = "community-altium-designer")]
	AltiumDesigner,
	#[cfg(feature = "community-auto-it")]
	AutoIt,
	#[cfg(feature = "community-automation-studio")]
	AutomationStudio,
	#[cfg(feature = "community-aws-cdk")]
	AwsCdk,
	#[cfg(feature = "community-aws-sam")]
	AwsSam,
	#[cfg(feature = "community-b4x")]
	B4x,
	#[cfg(feature = "community-bazel")]
	Bazel,
	#[cfg(feature = "community-beef")]
	Beef,
	#[cfg(feature = "community-dot-net-core")]
	DotNetCore,
	#[cfg(feature = "community-dot-net-infor-cms")]
	DotNetInforCms,
	#[cfg(feature = "community-dot-net-kentico")]
	DotNetKentico,
	#[cfg(feature = "community-dot-net-umbraco")]
	DotNetUmbraco,
	#[cfg(feature = "community-elixir-phoenix")]
	ElixirPhoenix,
	#[cfg(feature = "community-embedded-atmel-studio")]
	EmbeddedAtmelStudio,
	#[cfg(feature = "community-embedded-esp-idf")]
	EmbeddedEspIdf,
	#[cfg(feature = "community-embedded-iar-ewarm")]
	EmbeddedIarEwarm,
	#[cfg(feature = "community-embedded-u-vision")]
	EmbeddedUVision,
	#[cfg(feature = "community-exercism")]
	Exercism,
	#[cfg(feature = "community-gnome-gnome-shell-extension")]
	GnomeGnomeShellExtension,
	#[cfg(feature = "community-golang-go-allow-list")]
	GolangGoAllowList,
	#[cfg(feature = "community-golang-hugo")]
	GolangHugo,
	#[cfg(feature = "community-gretl")]
	Gretl,
	#[cfg(feature = "community-hexo")]
	Hexo,
	#[cfg(feature = "community-java-j-boss4")]
	JavaJBoss4,
	#[cfg(feature = "community-java-j-boss6")]
	JavaJBoss6,
	#[cfg(feature = "community-java-script-cordova")]
	JavaScriptCordova,
	#[cfg(feature = "community-java-script-meteor")]
	JavaScriptMeteor,
	#[cfg(feature = "community-java-script-n-wjs")]
	JavaScriptNWjs,
	#[cfg(feature = "community-java-script-vue")]
	JavaScriptVue,
	#[cfg(feature = "community-lens-studio")]
	LensStudio,
	#[cfg(feature = "community-linux-snap")]
	LinuxSnap,
	#[cfg(feature = "community-logtalk")]
	Logtalk,
	#[cfg(feature = "community-move")]
	Move,
	#[cfg(feature = "community-nasa-specs-intact")]
	NasaSpecsIntact,
	#[cfg(feature = "community-nix")]
	Nix,
	#[cfg(feature = "community-obsidian-notes-and-core-configuration")]
	ObsidianNotesAndCoreConfiguration,
	#[cfg(feature = "community-obsidian-notes-and-extended-configuration")]
	ObsidianNotesAndExtendedConfiguration,
	#[cfg(feature = "community-obsidian-notes-only")]
	ObsidianNotesOnly,
	#[cfg(feature = "community-open-ssl")]
	OpenSsl,
	#[cfg(feature = "community-open-tofu")]
	OpenTofu,
	#[cfg(feature = "community-php-bitrix")]
	PhpBitrix,
	#[cfg(feature = "community-php-code-sniffer")]
	PhpCodeSniffer,
	#[cfg(feature = "community-php-drupal7")]
	PhpDrupal7,
	#[cfg(feature = "community-php-jigsaw")]
	PhpJigsaw,
	#[cfg(feature = "community-php-magento1")]
	PhpMagento1,
	#[cfg(feature = "community-php-magento2")]
	PhpMagento2,
	#[cfg(feature = "community-php-pimcore")]
	PhpPimcore,
	#[cfg(feature = "community-php-think-php")]
	PhpThinkPhp,
	#[cfg(feature = "community-puppet")]
	Puppet,
	#[cfg(feature = "community-python-jupyter-notebooks")]
	PythonJupyterNotebooks,
	#[cfg(feature = "community-python-nikola")]
	PythonNikola,
	#[cfg(feature = "community-racket")]
	Racket,
	#[cfg(feature = "community-red")]
	Red,
	#[cfg(feature = "community-ros2")]
	Ros2,
	#[cfg(feature = "community-sp-fx")]
	SpFx,
	#[cfg(feature = "community-splunk")]
	Splunk,
	#[cfg(feature = "community-strapi")]
	Strapi,
	#[cfg(feature = "community-terragrunt")]
	Terragrunt,
	#[cfg(feature = "community-toit")]
	Toit,
	#[cfg(feature = "community-ui-path")]
	UiPath,
	#[cfg(feature = "community-v")]
	V,
	#[cfg(feature = "community-xilinx")]
	Xilinx,
}

impl GitIgnore for Community {
	#[cfg(feature = "no-contents")]
	fn contents(self) -> &'static str {
		""
	}

	#[cfg(not(feature = "no-contents"))]
	fn contents(self) -> &'static str {
		match self { #[cfg(feature = "community-alteryx")] Self::Alteryx => "# gitignore template for Alteryx Designer\n# website: https://www.alteryx.com/\n# website: https://help.alteryx.com/current/designer/alteryx-file-types\n\n# Alteryx Data Files\n*.yxdb\n*.cydb\n*.cyidx\n*.rptx\n*.vvf\n*.aws\n\n# Alteryx Special Files\n*.yxwv\n*.yxft\n*.yxbe\n*.bak\n*.pcxml\n*.log\n*.bin\n*.yxlang\nCASS.ini\n\n# Alteryx License Files\n*.yxlc\n*.slc\n*.cylc\n*.alc\n*.gzlc\n\n## gitignore reference sites\n# https://git-scm.com/book/en/v2/Git-Basics-Recording-Changes-to-the-Repository#_ignoring\n# https://git-scm.com/docs/gitignore\n# https://help.github.com/articles/ignoring-files/\n\n## Useful knowledge from stackoverflow\n# Even if you haven't tracked the files so far, git seems to be able to \"know\" about them even after you add them to .gitignore.\n# WARNING: First commit your current changes, or you will lose them.\n# Then run the following commands from the top folder of your git repo:\n# git rm -r --cached .\n# git add .\n# git commit -m \"fixed untracked files\"\n\n# author: Kacper Ksieski", #[cfg(feature = "community-altium-designer")] Self::AltiumDesigner => "# For PCBs designed using Altium Designer\n# Website: https://www.altium.com/altium-designer/\n\n# Directories containing cache data\nHistory\n__Previews\n\n# Directories containing logs and generated outputs\nProject\\ Logs*\nProject\\ Outputs*\n\n# Misc files generated by altium\ndebug.log\nStatus\\ Report.txt\n*.PcbDoc.htm\n*.SchDocPreview\n*.PcbDocPreview\n\n# Lock files sometimes left behind\n.~lock.*\n", #[cfg(feature = "community-auto-it")] Self::AutoIt => "# Compiled Scripts\n*.a3x\n\n# Tidy Auto-Generated Backups\nBackup/*\n\n# Au3Stripper Auto-Generated Files\n*_stripped.au3\n", #[cfg(feature = "community-automation-studio")] Self::AutomationStudio => "# gitignore template for B&R Automation Studio (AS) 4\n# website: https://www.br-automation.com/en-us/products/software/automation-software/automation-studio/\n\n# AS temporary directories\nBinaries/\nDiagnosis/\nTemp/\nTempObjects/\n\n# AS transfer files\n*artransfer.br\n*arTrsfmode.nv\n\n# 'ignored' directory\nignored/\n\n# ARNC0ext\n*arnc0ext.br\n\n# AS File types\n*.bak\n*.isopen\n*.orig\n*.log\n*.asar\n*.csvlog*\n*.set\n!**/Physical/**/*.set\n\n# RevInfo variables\n*RevInfo.var\n", #[cfg(feature = "community-aws-cdk")] Self::AwsCdk => "# CDK asset staging directory.\n# For more information about AWS-CDK, see  https://docs.aws.amazon.com/cdk/\n.cdk.staging/\ncdk.out/\n", #[cfg(feature = "community-aws-sam")] Self::AwsSam => "# gitignore template for AWS Serverless Application Model project\n# website: https://docs.aws.amazon.com/serverless-application-model\n\n# Ignore build folder\n.aws-sam/\n", #[cfg(feature = "community-b4x")] Self::B4x => "**/Objects\n**/AutoBackups\n*.meta\n", #[cfg(feature = "community-bazel")] Self::Bazel => "# gitignore template for Bazel build system\n# website: https://bazel.build/\n\n# Ignore all bazel-* symlinks. There is no full list since this can change\n# based on the name of the directory bazel is cloned into.\n/bazel-*\n\n# Directories for the Bazel IntelliJ plugin containing the generated\n# IntelliJ project files and plugin configuration. Seperate directories are\n# for the IntelliJ, Android Studio and CLion versions of the plugin.\n/.ijwb/\n/.aswb/\n/.clwb/\n", #[cfg(feature = "community-beef")] Self::Beef => "build/\nrecovery/\nBeefSpace_User.toml\n", #[cfg(feature = "community-dot-net-core")] Self::DotNetCore => "*.swp\n*.*~\nproject.lock.json\n.DS_Store\n*.pyc\nnupkg/\n\n# Visual Studio Code\n.vscode\n\n# Rider\n.idea\n\n# User-specific files\n*.suo\n*.user\n*.userosscache\n*.sln.docstates\n\n# Build results\n[Dd]ebug/\n[Dd]ebugPublic/\n[Rr]elease/\n[Rr]eleases/\nx64/\nx86/\nbuild/\nbld/\n[Bb]in/\n[Oo]bj/\n[Oo]ut/\nmsbuild.log\nmsbuild.err\nmsbuild.wrn\n\n# Visual Studio 2015\n.vs/\n\n", #[cfg(feature = "community-dot-net-infor-cms")] Self::DotNetInforCms => "# gitignore template for InforCRM (formerly SalesLogix)\n# website: https://www.infor.com/product-summary/cx/infor-crm/\n#\n# Recommended: VisualStudio.gitignore\n\n# Ignore model files that are auto-generated\nModelIndex.xml\nExportedFiles.xml\n\n# Ignore deployment files\n[Mm]odel/[Dd]eployment\n\n# Force include portal SupportFiles\n!Model/Portal/*/SupportFiles/[Bb]in/\n!Model/Portal/PortalTemplates/*/SupportFiles/[Bb]in\n", #[cfg(feature = "community-dot-net-kentico")] Self::DotNetKentico => "# gitignore template for using Kentico CMS\n# website: http://www.kentico.com/\n#\n# Recommended template: VisualStudio.gitignore\n\n# Include some Kentico folders excluded by Visual Studio rules\n!CMS/CMSAdminControls/*/\n!CMS/CMSModules/System/*/\n!CMS/App_Data/CIRepository/**\n\n# Kentico temporary/environment files\nCMS/App_Data/AzureCache\nCMS/App_Data/AzureTemp\nCMS/App_Data/CMSModules/DeviceProfile/logFiftyOne.txt\nCMS/App_Data/CMSModules/DeviceProfiles/logFiftyOne.txt\nCMS/App_Data/CMSModules/WebFarm/webfarm.sync\nCMS/App_Data/CMSTemp\nCMS/App_Data/Persistent\nCMS/CMSSiteUtils/Export\nCMS/CMSSiteUtils/Import\n\n# Ignore all smart search indexes, but not the other system folder contents\nCMS/App_Data/CMSModules/SmartSearch/**\n!CMS/App_Data/CMSModules/SmartSearch/*/\n!CMS/App_Data/CMSModules/SmartSearch/_StopWords/**\n!CMS/App_Data/CMSModules/SmartSearch/_Synonyms/**\n\n## Kentico Starter Sites\n# Starter site resource Files\nCMS/App_Data/DancingGoat\n\n# Starter site web templates\nCMS/App_Data/Templates/CommunitySite\nCMS/App_Data/Templates/CorporateSite\nCMS/App_Data/Templates/DancingGoat\nCMS/App_Data/Templates/EcommerceSite\nCMS/App_Data/Templates/IntranetPortal\nCMS/App_Data/Templates/PersonalSite\n\n# Starter site app themes\nCMS/App_Themes/CommunitySite\nCMS/App_Themes/CorporateSite\nCMS/App_Themes/EcommerceSite\nCMS/App_Themes/IntranetPortal*\nCMS/App_Themes/PersonalSite\n\n# Starter site ASPX templates\nCMS/CMSTemplates/CorporateSite\n\n# Starter site media libraries\nCMS/CommunitySite\nCMS/CorporateSite\nCMS/DancingGoat\nCMS/EcommerceSite\nCMS/IntranetPortal\nCMS/PersonalSite\n\n## Project specific ignores\n# Sensitive settings\nAppSettings.config\nConnectionStrings.config\n\n# Project media libraries (recommend shared file storage)\n# e.g. CMS/{SiteCodeName}\n", #[cfg(feature = "community-dot-net-umbraco")] Self::DotNetUmbraco => "## Ignore Umbraco files/folders generated for each instance\n##\n## Get latest from https://github.com/github/gitignore/blob/main/Umbraco.gitignore\n\n# Note: VisualStudio gitignore rules may also be relevant\n\n# Umbraco\n# Ignore unimportant folders generated by Umbraco\n**/App_Data/Logs/\n**/App_Data/[Pp]review/\n**/App_Data/TEMP/\n**/App_Data/NuGetBackup/\n\n# Ignore Umbraco content cache file\n**/App_Data/umbraco.config\n\n## this [Uu]mbraco/ folder should be created by cmd like `Install-Package UmbracoCms -Version 8.5.3`\n## you can find your Umbraco version in your Web.config. (i.e. <add key=\"Umbraco.Core.ConfigurationStatus\" value=\"8.5.3\" />)\n## Uncomment this line if you think it fits the way you work on your project.\n## **/[Uu]mbraco/\n\n## The [Mm]edia/ folder contains content. Content may vary by environment and should therefore not be added to source control.\n## Uncomment this line if you think it fits the way you work on your project.\n## **/[Mm]edia/ \n\n# Don't ignore Umbraco packages (VisualStudio.gitignore mistakes this for a NuGet packages folder)\n# Make sure to include details from VisualStudio.gitignore BEFORE this\n!**/App_Data/[Pp]ackages/*\n!**/[Uu]mbraco/[Dd]eveloper/[Pp]ackages/*\n!**/[Uu]mbraco/[Vv]iews/[Pp]ackages/*\n\n# ImageProcessor DiskCache\n**/App_Data/cache/\n\n# Ignore the Models Builder models out of date flag\n**/ood.flag\n\n# NEW for version 9 .Net 5 (Core)\n#ignore umbraco backoffice assest from wwwroot\n**/wwwroot/umbraco/\n\n# SQLite files\n*.sqlite.db*\n\n#ignore umbraco data/views/settings\n**/umbraco/\n\n#include default location for modelsbuilder output\n!**/umbraco/models\n\n#include default location for packages\n!**/umbraco/Data/packages\n", #[cfg(feature = "community-elixir-phoenix")] Self::ElixirPhoenix => "# gitignore template for Phoenix projects\n# website: http://www.phoenixframework.org/\n#\n# Recommended template: Elixir.gitignore\n\n# Temporary files\n/tmp\n\n# Static artifacts\n/node_modules\n/assets/node_modules\n\n# Since we are building assets from web/static,\n# we ignore priv/static. You may want to comment\n# this depending on your deployment strategy.\n/priv/static/\n\n# Installer-related files\n/installer/_build\n/installer/tmp\n/installer/doc\n/installer/deps\n", #[cfg(feature = "community-embedded-atmel-studio")] Self::EmbeddedAtmelStudio => "## Ignore Atmel Studio temporary files and build results\n# https://www.microchip.com/mplab/avr-support/atmel-studio-7\n\n# Atmel Studio is powered by an older version of Visual Studio,\n# so most of the project and solution files are the same as VS files,\n# only prefixed by an `at`.\n\n#Build Directories\n[Dd]ebug/\n[Rr]elease/\n\n#Build Results\n*.o\n*.d\n*.eep\n*.elf\n*.hex\n*.map\n*.srec\n\n#User Specific Files\n*.atsuo\n", #[cfg(feature = "community-embedded-esp-idf")] Self::EmbeddedEspIdf => "# gitignore template for esp-idf, the official development framework for ESP32\n# https://github.com/espressif/esp-idf\n\nbuild/\nsdkconfig\nsdkconfig.old\n", #[cfg(feature = "community-embedded-iar-ewarm")] Self::EmbeddedIarEwarm => "# gitignore template for the IAR EWARM\n# website: https://www.iar.com/knowledge/support/technical-notes/ide/which-files-should-be-version-controlled/\n\n# Some tools will put the EWARM files\n# under a subdirectory with the same name\n# as the configuration.\n# Example\n# EWARM/Config1/Obj /List /Exe\n# EWARM/Config2/Obj /List /Exe\nEWARM/**/Obj\nEWARM/**/List\nEWARM/**/Exe\n\n# Autogenerated project files\n*.dep\n*.ewt\n\n# Autogenerated folder for debugger\nEWARM/settings\n", #[cfg(feature = "community-embedded-u-vision")] Self::EmbeddedUVision => "# git ignore file for Keil µVision Project\n\n# µVision 5 and µVision 4 Project screen layout file\n*.uvguix.*\n*.uvgui.*\n\n# Listing Files\n*.i\n*.lst\n*.m51\n*.m66\n*.map\n\n# Object Files\n*.axf\n*.b[0-2][0-9]\n*.b3[0-1]\n*.bak\n*.build_log.htm\n*.crf\n*.d\n*.dep\n*.elf\n*.htm\n*.iex\n*.lnp\n*.o\n*.obj\n*.sbr\n\n# Firmware Files\n*.bin\n*.h86\n*.hex\n\n# Build Files\n.bat\n\n# Debugger Files\n.ini\n\n# JLink Files\nJLinkLog.txt\n\n# Other Files\n", #[cfg(feature = "community-exercism")] Self::Exercism => "# gitignore template for Exercism project\n# website: https://exercism.io/\n\n# Ignore .exercism folder which contain sensitive data\n.exercism\n", #[cfg(feature = "community-gnome-gnome-shell-extension")] Self::GnomeGnomeShellExtension => "# Ignored files for GNOME extension git repository\n\n*.zip\n", #[cfg(feature = "community-golang-go-allow-list")] Self::GolangGoAllowList => "# Allowlisting gitignore template for GO projects prevents us\n# from adding various unwanted local files, such as generated\n# files, developer configurations or IDE-specific files etc.\n#\n# Recommended: Go.AllowList.gitignore\n\n# Ignore everything\n*\n\n# But not these files...\n!/.gitignore\n\n!*.go\n!go.sum\n!go.mod\n\n!README.md\n!LICENSE\n\n# !Makefile\n\n# ...even if they are in subdirectories\n!*/\n", #[cfg(feature = "community-golang-hugo")] Self::GolangHugo => "# Generated files by hugo\n/public/\n/resources/_gen/\n/assets/jsconfig.json\nhugo_stats.json\n\n# Executable may be added to repository\nhugo.exe\nhugo.darwin\nhugo.linux\n\n# Temporary lock file while building\n/.hugo_build.lock\n", #[cfg(feature = "community-gretl")] Self::Gretl => "# gitignore template for Gretl\n# website: http://gretl.sourceforge.net/\n\n# Auto-generated log file is overwritten whenever you start a new session\nsession.inp\n\n# Auto-generated temporary string code table\nstring_table.txt\n", #[cfg(feature = "community-hexo")] Self::Hexo => "# gitignore template for Hexo sites\n# website: https://hexo.io/\n# Recommended: Node.gitignore\n\n# Ignore generated directory\npublic/\n\n# Ignore temp files\ntmp/\n.tmp*\n\n# additional files\ndb.json\n.deploy*/\n", #[cfg(feature = "community-java-j-boss4")] Self::JavaJBoss4 => "# gitignore for JBoss v4 projects\n\n/server/all/data\n/server/all/log\n/server/all/tmp\n/server/all/work\n/server/default/data\n/server/default/log\n/server/default/tmp\n/server/default/work\n/server/minimal/data\n/server/minimal/log\n/server/minimal/tmp\n/server/minimal/work\n\n# Note:\n# there may be other directories that contain *.xml.failed or *.war.failed files\n/server/default/deploy/*.xml.failed\n/server/default/deploy/*.war.failed\n", #[cfg(feature = "community-java-j-boss6")] Self::JavaJBoss6 => "# gitignore for JBoss v6 projects\n#\n# Note: to ensure empty directories remain part of the repository, like\n# `/server/minimal/lib`, you should add an empty `.gitignore` or `.gitkeep` file\n# to the directory - otherwise you may have issues when starting the service.\n\n/server/all/data\n/server/all/log\n/server/all/tmp\n/server/all/work\n/server/default/data\n/server/default/log\n/server/default/tmp\n/server/default/work\n/server/minimal/data\n/server/minimal/log\n/server/minimal/tmp\n/server/minimal/work\n/server/jbossweb-standalone/data\n/server/jbossweb-standalone/log\n/server/jbossweb-standalone/tmp\n/server/jbossweb-standalone/work\n/server/standard/data\n/server/standard/log\n/server/standard/tmp\n/server/standard/work\n/server/default/deploy/*.jar.failed\n/server/default/deploy/*.jar.dodeploy\n/server/default/deploy/*.xml.failed\n/server/default/deploy/*.xml.dodeploy\n/server/default/deploy/*.war.failed\n/server/default/deploy/*.war.dodeploy\n\n", #[cfg(feature = "community-java-script-cordova")] Self::JavaScriptCordova => "# gitignore template for the Cordova framework\n# website: https://cordova.apache.org/\n#\n# Recommended template: Node.gitignore\n\n# App platform binaries and built files\n/platforms\n\n# Optional to ignore plugin Git clones\n#/plugins\n", #[cfg(feature = "community-java-script-meteor")] Self::JavaScriptMeteor => "# gitignore template for the Meteor framework\n# website: https://www.meteor.com/\n#\n# Recommended template: Node.gitignore\n\n# protect api keys in setting json\nsettings-production.json\nsettings.json\n\n# protect your mup.json settings\nmup.json\nmup.js\n", #[cfg(feature = "community-java-script-n-wjs")] Self::JavaScriptNWjs => "# gitignore template for NW.js projects\n# website: https://nwjs.io/\n\n# Seen in standard and sdk versions\ncredits.html\nlocales/\nlibEGL.dll\nlibGLEv2.dll\nnode.dll\nnw.dll\nnw.exe\nnatives_blob.bin\nnw_100_percent.pak\nnw_200_percent.pak\nnw_elf.dll\nsnapshot_blob.bin\nresources.pak\n\n# Seen only in standard\nd3dcompiler_47.dll\nffmpeg.dll\nicudtl.dat\n\n# Seen only in sdk\npnacl/\nchromedriver.exe\nnacl_irt_x86_64.nexe\nnwjc.exe\npayload.exe\n", #[cfg(feature = "community-java-script-vue")] Self::JavaScriptVue => "# gitignore template for Vue.js projects\n#\n# Recommended template: Node.gitignore\n\n# TODO: where does this rule come from?\ndocs/_book\n\n# TODO: where does this rule come from?\ntest/\n", #[cfg(feature = "community-lens-studio")] Self::LensStudio => "# gitignore template for LensStudio\n# website: https://lensstudio.snapchat.com/\n\n# macOS/IDE #\n.DS_Store\n.idea\n\n# js #\nnode_modules\nyarn.lock\n\n# Python #\n__pycache__/\n*.py[cod]\n*$py.class\n[Bb]ackup*\n", #[cfg(feature = "community-linux-snap")] Self::LinuxSnap => "# gitginore template for creating Snap packages\n# website: https://snapcraft.io/\n\nparts/\nprime/\nstage/\n*.snap\n\n# Snapcraft global state tracking data(automatically generated)\n# https://forum.snapcraft.io/t/location-to-save-global-state/768\n/snap/.snapcraft/\n\n# Source archive packed by `snapcraft cleanbuild` before pushing to the LXD container\n/*_source.tar.bz2\n", #[cfg(feature = "community-logtalk")] Self::Logtalk => "# gitignore template for LogTalk, a programming language that builds upon Prolog\n# website: https://logtalk.org/\n\n# Logtalk temporary file directories\n.lgt_tmp/\nlgt_tmp/\n\n# Logtalk default unit testing and doclet results and logs directories\nlogtalk_tester_logs/\nlogtalk_doclet_logs/\n\n# backend Prolog compiler temporary files\n.pl-history\n*.out\n*.xwam\n*.qo\n*.ql\n*.itf\n*.po\n", #[cfg(feature = "community-move")] Self::Move => "# Generated by Move\n# will have compiled files\nbuild/\n\n# Remove possibly saving credentials to the git repository\n.aptos/\n", #[cfg(feature = "community-nasa-specs-intact")] Self::NasaSpecsIntact => "# gitignore template for Nasa SpecsIntact (SI)\n# Website: https://specsintact.ksc.nasa.gov/\n#\n# Recommended: \n# MicrosoftOffice.gitignore\n# \n\n# SpecsIntact (SI) Locking file; this would lock everyone out.\n*.se$\n\n# SI Reports; auto-generated. They do not belong in the repository\n# as they will be re-created exactly when using a specific checkout point.\n*.RPT\nADDRVER.*\nBRKTVER.*\nDUPEREF.*\nREFVER.*\nSECTVER.*\nSUBMVER.*\nTTLDIFFS.*\n\n# SpecsIntact files that change a lot and don't actually affect SI\n# PULL.TBL is an auto-generated file to help speed SI loading. \nPULL.TBL\npulltbl.bck\n\n# Tailoring information.\n# Keep tailor.tag; it is a list of tailoring options in SI.\n\n# JOB.OTL informs SI where a spec section came from. \n# Keeping the old one isn't useful in git.\nJOB.OTL.OLD\n\n# OneNote TOC Files; SI Work Directories may be installed in a location co-located with OneNote\n# notebooks, and if so, OneNote will litter the SI folder with these.\n*.onetoc*\n\n# Log files, typically tagfix or other auto generated logs that aren't useful \n# outside of the user that made them and clutter up the index.\n*.log\n", #[cfg(feature = "community-nix")] Self::Nix => "# Ignore build outputs from performing a nix-build or `nix build` command\nresult\nresult-*\n\n# Ignore automatically generated direnv output\n.direnv\n", #[cfg(feature = "community-obsidian-notes-and-core-configuration")] Self::ObsidianNotesAndCoreConfiguration => "# Excludes Obsidian workspace cache and plugins. All notes and core obsidian\n# configuration files are tracked by Git.\n\n# The current application UI state (DOM layout, recently-opened files, etc.) is\n# stored in these files (separate for desktop and mobile) so you can resume\n# your session seamlessly after a restart. If you want to track UI state, use\n# the Workspaces core plugin instead of relying on these files.\n.obsidian/workspace.json\n.obsidian/workspace-mobile.json\n\n# Obsidian plugins are stored under .obsidian/plugins/$plugin_name. They\n# contain metadata (manifest.json), application code (main.js), stylesheets\n# (styles.css), and user-configuration data (data.json).\n# We want to exclude all plugin-related files, so we can exclude everything\n# under this directory.\n.obsidian/plugins/**/*\n", #[cfg(feature = "community-obsidian-notes-and-extended-configuration")] Self::ObsidianNotesAndExtendedConfiguration => "# Excludes Obsidian workspace cache and plugin code, but retains plugin\n# configuration. All notes and user-controlled configuration files are tracked\n# by Git.\n#\n# \t\t\t\t!!! WARNING !!!\n#\n# Community plugins may store sensitive secrets in their data.json files. By\n# including these files, those secrets may be tracked in your Git repository.\n#\n# To ignore configurations for specific plugins, add a line like this after the\n# contents of this file (order is important):\n#     .obsidian/plugins/{{plugin_name}}/data.json\n#\n# Alternatively, ensure that you are treating your entire Git repository as\n# sensitive data, since it may contain secrets, or may have contained them in\n# past commits.  Understand your threat profile, and make the decision\n# appropriate for yourself. If in doubt, err on the side of not including\n# plugin configuration. Use one of the alternative gitignore files instead:\n# * NotesOnly.gitignore\n# * NotesAndCoreConfiguration.gitignore\n\n# The current application UI state (DOM layout, recently-opened files, etc.) is\n# stored in these files (separate for desktop and mobile) so you can resume\n# your session seamlessly after a restart. If you want to track UI state, use\n# the Workspaces core plugin instead of relying on these files.\n.obsidian/workspace.json\n.obsidian/workspace-mobile.json\n\n# Obsidian plugins are stored under .obsidian/plugins/$plugin_name. They\n# contain metadata (manifest.json), application code (main.js), stylesheets\n# (styles.css), and user-configuration data (data.json).\n# We only want to track data.json, so we:\n# 1. exclude everything under the plugins directory recursively,\n# 2. unignore the plugin directories themselves, which then allows us to\n# 3. unignore the data.json files\n.obsidian/plugins/**/*\n!.obsidian/plugins/*/\n!.obsidian/plugins/*/data.json\n", #[cfg(feature = "community-obsidian-notes-only")] Self::ObsidianNotesOnly => "# Excludes all Obsidian-related configuration. All notes are tracked by Git.\n\n# All Obsidian configuration and runtime state is stored here\n.obsidian/**/*\n", #[cfg(feature = "community-open-ssl")] Self::OpenSsl => "# OpenSSL-related files best not committed\n\n## Certificate Authority\n*.ca\n\n## Certificate\n*.crt\n\n## Certificate Sign Request\n*.csr\n\n## Certificate\n*.der\n\n## Key database file\n*.kdb\n\n## OSCP request data\n*.org\n\n## PKCS #12\n*.p12\n\n## PEM-encoded certificate data\n*.pem\n\n## Random number seed\n*.rnd\n\n## SSLeay data\n*.ssleay\n\n## S/MIME message\n*.smime\n", #[cfg(feature = "community-open-tofu")] Self::OpenTofu => "# Local .terraform directories\n**/.terraform/*\n\n# .tfstate files\n*.tfstate\n*.tfstate.*\n\n# Crash log files\ncrash.log\ncrash.*.log\n\n# Exclude all .tfvars files, which are likely to contain sensitive data, such as\n# password, private keys, and other secrets. These should not be part of version \n# control as they are data points which are potentially sensitive and subject \n# to change depending on the environment.\n*.tfvars\n*.tfvars.json\n\n# Ignore override files as they are usually used to override resources locally and so\n# are not checked in\noverride.tf\noverride.tofu\noverride.tf.json\noverride.tofu.json\n*_override.tf\n*_override.tofu\n*_override.tf.json\n*_override.tofu.json\n\n# Ignore transient lock info files created by tofu apply\n.terraform.tfstate.lock.info\n\n# Include override files you do wish to add to version control using negated pattern\n# !example_override.tf\n# !example_override.tofu\n\n# Include tfplan files to ignore the plan output of command: tofu plan -out=tfplan\n# example: *tfplan*\n\n# Ignore CLI configuration files\n.terraformrc\nterraform.rc\n", #[cfg(feature = "community-php-bitrix")] Self::PhpBitrix => "# gitignore template for 1C-Bitrix, a PHP-based CMS\n# website: https://www.1c-bitrix.ru\n\n#Exclude all of core files\n/bitrix/*\n\n#But not the templates and non bitrix components\n!/bitrix/templates\n!/bitrix/components\n/bitrix/components/bitrix\n\n#Exclude bitrix gadgets\n!/bitrix/gadgets\n/bitrix/gadgets/bitrix\n\n#User can use that directory to store some stuff, but it's not really recommended, just use /local instead of this\n!/bitrix/php_interface/\n\n#Exclude database configs\n/bitrix/php_interface/dbconn.php\n\n#Exclude default file storage directory\n/upload/\n", #[cfg(feature = "community-php-code-sniffer")] Self::PhpCodeSniffer => "# gitignore for the PHP Codesniffer framework\n# website: https://github.com/squizlabs/PHP_CodeSniffer\n#\n# Recommended template: PHP.gitignore\n\n/wpcs/*\n", #[cfg(feature = "community-php-drupal7")] Self::PhpDrupal7 => "# gitignore template for Drupal 7 projects\n#\n# It is recommended that you use `Drupal.gitignore` as this is the latest version\n\n# Ignore configuration files that may contain sensitive information.\nsites/*/*settings*.php\nsites/example.sites.php\n\n# Ignore paths that contain generated content.\nfiles/\nsites/*/files\nsites/*/private\nsites/*/translations\n\n# Ignore default text files\nrobots.txt\n/CHANGELOG.txt\n/COPYRIGHT.txt\n/INSTALL*.txt\n/LICENSE.txt\n/MAINTAINERS.txt\n/UPGRADE.txt\n/README.txt\nsites/README.txt\nsites/all/libraries/README.txt\nsites/all/modules/README.txt\nsites/all/themes/README.txt\n\n# Ignore everything but the \"sites\" folder ( for non core developer )\n.htaccess\nweb.config\nauthorize.php\ncron.php\nindex.php\ninstall.php\nupdate.php\nxmlrpc.php\n/includes\n/misc\n/modules\n/profiles\n/scripts\n/themes\n", #[cfg(feature = "community-php-jigsaw")] Self::PhpJigsaw => "# gitignore template for Jigsaw Static Site Generator\n#\n# website - https://jigsaw.tighten.co\n\n# Ignore build folder\nbuild_*\n", #[cfg(feature = "community-php-magento1")] Self::PhpMagento1 => "# gitignore template for Magento v1 projects\n#\n# It is recommended that you use `Magento.gitignore` as this is the latest version\n\n/PATCH_*.sh\n\n/app/etc/local.xml\n\n/media/*\n!/media/.htaccess\n\n!/media/customer\n/media/customer/*\n!/media/customer/.htaccess\n\n!/media/dhl\n/media/dhl/*\n!/media/dhl/logo.jpg\n\n!/media/downloadable\n/media/downloadable/*\n!/media/downloadable/.htaccess\n\n!/media/xmlconnect\n/media/xmlconnect/*\n\n!/media/xmlconnect/custom\n/media/xmlconnect/custom/*\n!/media/xmlconnect/custom/ok.gif\n\n!/media/xmlconnect/original\n/media/xmlconnect/original/*\n!/media/xmlconnect/original/ok.gif\n\n!/media/xmlconnect/system\n/media/xmlconnect/system/*\n!/media/xmlconnect/system/ok.gif\n\n/var/*\n!/var/.htaccess\n\n!/var/package\n/var/package/*\n!/var/package/*.xml\n\n", #[cfg(feature = "community-php-magento2")] Self::PhpMagento2 => "/sitemap\n/sitemap.xml\n/pub/sitemap\n/pub/sitemap.xml\n/app/config_sandbox\n/app/etc/config.php\n/app/etc/env.php\n/app/code/Magento/TestModule*\n/lib/internal/flex/uploader/.actionScriptProperties\n/lib/internal/flex/uploader/.flexProperties\n/lib/internal/flex/uploader/.project\n/lib/internal/flex/uploader/.settings\n/lib/internal/flex/varien/.actionScriptProperties\n/lib/internal/flex/varien/.flexLibProperties\n/lib/internal/flex/varien/.project\n/lib/internal/flex/varien/.settings\n/.grunt\n/.php_cs.cache\n/grunt-config.json\n/dev/tools/grunt/configs/local-themes.js\n\n/pub/media/*.*\n!/pub/media/.htaccess\n/pub/media/attribute/*\n!/pub/media/attribute/.htaccess\n/pub/media/analytics/*\n/pub/media/catalog/*\n!/pub/media/catalog/.htaccess\n/pub/media/customer/*\n!/pub/media/customer/.htaccess\n/pub/media/downloadable/*\n!/pub/media/downloadable/.htaccess\n/pub/media/favicon/*\n/pub/media/import/*\n!/pub/media/import/.htaccess\n/pub/media/logo/*\n/pub/media/theme/*\n/pub/media/theme_customization/*\n!/pub/media/theme_customization/.htaccess\n/pub/media/wysiwyg/*\n!/pub/media/wysiwyg/.htaccess\n/pub/media/tmp/*\n!/pub/media/tmp/.htaccess\n/pub/media/captcha/*\n!/pub/media/captcha/.htaccess\n/pub/static/*\n!/pub/static/.htaccess\n\n/var/*\n!/var/.htaccess\n/vendor/*\n!/vendor/.htaccess\n/generated/*\n!/generated/.htaccess\n", #[cfg(feature = "community-php-pimcore")] Self::PhpPimcore => "# gitignore template for Pimcore CMS\n\n# pimcore source files\n/pimcore\n\n# asset files\n/website/var/assets/*\n\n# backups\n/website/var/backup/*\n\n# file cache\n/website/var/cache/*\n\n# generated PHP classes, keep definition files (.psf)\n/website/var/classes/Object*\n!/website/var/classes/objectbricks\n\n# various configuration files\n/website/var/config/system.xml\n/website/var/config/cache.xml\n/website/var/config/robots.txt\n/website/var/config/Geo*\n/website/var/config/object/*\n/website/var/config/portal/*\n/website/var/config/sqlreport/*\n\n# sent e-mail log files\n/website/var/email/*\n\n# log files\n/website/var/log/*.log\n\n# serialized recyclebin files\n/website/var/recyclebin/*\n\n# search plugin\n/website/var/search/*\n\n# various temp files\n/website/var/system/*\n/website/var/tmp/*\n\n# serialized version files\n/website/var/versions/asset/*\n/website/var/versions/document/*\n/website/var/versions/object/*\n\n# user profile images\n/website/var/user-image/*\n\n# keep .dummy files\n!.dummy\n", #[cfg(feature = "community-php-think-php")] Self::PhpThinkPhp => "# gitignore template for ThinkPHP v3.2.3\n# website: http://www.thinkphp.cn/\n\n# Logs and Cache files\n/Application/Runtime/\n\n# Common configure file\n/Application/Common/Conf/config.php", #[cfg(feature = "community-puppet")] Self::Puppet => "# gitignore template for Puppet modules\n# website: https://forge.puppet.com/\n\n# Built packages\npkg/*\n\n# Should run on multiple platforms so don't check in\nGemfile.lock\n\n# Tests\nspec/fixtures/*\ncoverage/*\n\n# Third-party\nvendor/*\n.bundle/*\n", #[cfg(feature = "community-python-jupyter-notebooks")] Self::PythonJupyterNotebooks => "# gitignore template for Jupyter Notebooks\n# website: http://jupyter.org/\n\n.ipynb_checkpoints\n*/.ipynb_checkpoints/*\n\n# IPython\nprofile_default/\nipython_config.py\n\n# Remove previous ipynb_checkpoints\n#   git rm -r .ipynb_checkpoints/\n", #[cfg(feature = "community-python-nikola")] Self::PythonNikola => "# gitignore template for Nikola static site generator\n# website: https://getnikola.com/\n\n.doit.db\n*.py[cod]\ncache/\noutput/\n", #[cfg(feature = "community-racket")] Self::Racket => "# gitignore template for the Racket language\n# website: http://www.racket-lang.org/\n\n# DrRacket autosave files\n*.rkt~\n*.rkt.bak\n\\#*.rkt#\n\\#*.rkt#*#\n\n# Compiled racket bytecode\ncompiled/\n*.zo\n\n# Dependency tracking files\n*.dep\n", #[cfg(feature = "community-red")] Self::Red => "# gitignore template for Red programming language\n# website: http://www.red-lang.org/\n\n# Red Compiled code\n*.red\n\n# Libraries\ncrush.dll\ncrush.dylib\ncrush.so\n\n# Files generated during test\nquick-test/quick-test.log\nquick-test/runnable/\nsystem/tests/source/units/auto-tests/\ntests/source/units/auto-tests/\n", #[cfg(feature = "community-ros2")] Self::Ros2 => "install/\nlog/\nbuild/\n\n# Ignore generated docs\n*.dox\n*.wikidoc\n\n# eclipse stuff\n.project\n.cproject\n\n# qcreator stuff\nCMakeLists.txt.user\n\nsrv/_*.py\n*.pcd\n*.pyc\nqtcreator-*\n*.user\n\n*~\n\n# Emacs\n.#*\n\n# Colcon custom files\nCOLCON_IGNORE\nAMENT_IGNORE\n", #[cfg(feature = "community-sp-fx")] Self::SpFx => "#SharePoint Framework (SPFx)\n# Logs\nlogs\n*.log\nnpm-debug.log*\n\n# Dependency directories\nnode_modules\n\n# Build generated files\ndist\nlib\nsolution\ntemp\n*.sppkg\n\n# Coverage directory used by tools like istanbul\ncoverage\n\n# OSX\n.DS_Store\n\n# Visual Studio files\n.ntvs_analysis.dat\n.vs\nbin\nobj\n\n# Resx Generated Code\n*.resx.ts\n\n# Styles Generated Code\n*.scss.ts\n", #[cfg(feature = "community-splunk")] Self::Splunk => "# gitignore template for Splunk apps\n# documentation: http://docs.splunk.com/Documentation/Splunk/6.2.3/admin/Defaultmetaconf\n\n# Splunk local meta file\nlocal.meta\n\n# Splunk local folder\nlocal\n", #[cfg(feature = "community-strapi")] Self::Strapi => "############################\n# OS X\n############################\n\n.DS_Store\n.AppleDouble\n.LSOverride\nIcon\n.Spotlight-V100\n.Trashes\n._*\n\n\n############################\n# Linux\n############################\n\n*~\n\n\n############################\n# Windows\n############################\n\nThumbs.db\nehthumbs.db\nDesktop.ini\n$RECYCLE.BIN/\n*.cab\n*.msi\n*.msm\n*.msp\n\n\n############################\n# Packages\n############################\n\n*.7z\n*.csv\n*.dat\n*.dmg\n*.gz\n*.iso\n*.jar\n*.rar\n*.tar\n*.zip\n*.com\n*.class\n*.dll\n*.exe\n*.o\n*.seed\n*.so\n*.swo\n*.swp\n*.swn\n*.swm\n*.out\n*.pid\n\n\n############################\n# Logs and databases\n############################\n\n.tmp\n*.log\n*.sql\n*.sqlite\n\n\n############################\n# Misc.\n############################\n\n*#\n.idea\nnbproject\n.vscode/\n\n\n############################\n# Node.js\n############################\n\nlib-cov\nlcov.info\npids\nlogs\nresults\nbuild\nnode_modules\n.node_history\npackage-lock.json\n**/package-lock.json\n!docs/package-lock.json\n*.heapsnapshot\n\n\n############################\n# Tests\n############################\n\ntestApp\ncoverage\ncypress/screenshots\ncypress/videos\n\n\n############################\n# Documentation\n############################\n\ndist\n\n############################\n# Builds\n############################\n\npackages/strapi-generate-new/files/public/\n\n############################\n# Example app\n############################\n\n.dev\n# *.cache\n\n############################\n# Visual Studio Code\n############################\n\nfront-workspace.code-workspace\n", #[cfg(feature = "community-terragrunt")] Self::Terragrunt => "# Ignore the default terragrunt cache directory\n# https://terragrunt.gruntwork.io/docs/features/caching/\n.terragrunt-cache\n", #[cfg(feature = "community-toit")] Self::Toit => ".packages\n*_pb.toit\n", #[cfg(feature = "community-ui-path")] Self::UiPath => "# gitignore template for RPA development using UiPath Studio \r\n# website: https://www.uipath.com/product/studio\r\n#\r\n# Recommended: n/a\r\n\r\n# Ignore folders that could cause issues if accidentally tracked\r\n**/.local/**\r\n**/.settings/**\r\n**/.objects/**\r\n**/.tmh/**\r\n**/*.log\r\n", #[cfg(feature = "community-v")] Self::V => "*.exe\n*.o\n*.so\n*.tmp.c\n*.exp\n*.ilk\n*.pdb\n*.dll\n*.lib\n*.bak\n*.out\n", #[cfg(feature = "community-xilinx")] Self::Xilinx => "# gitignore template for Xilinx Vivado Design Suite\n# website: https://www.xilinx.com/support/download.html\n\n# [home]\n*.jou\n*.log\n*.debug\n*.str\n*.zip\n*.tmp\n*.rst\n*.os\n*.js\n*.pb\n*.dcp\n*.hwdef\n*.vds\n*.veo\n*.wdf\n*.vdi\n*.dmp\n*.rpx\n*.rpt\n*_stub.v\n*_stub.vhdl\n*_funcsim.v\n*_funcsim.vhdl\n.project\n\n# [dir]\n*.cache\n.metadata\n*.data\n*.ipdefs\n.Xil\n*.sdk\n*.hw\n*.ip_user_files\n\n### IP synth\n*_synth_*\n\n.jobs\n\n### project synth\n*/*.runs/synth*/*.xml\n*/*.runs/synth*/*.txt\n*/*.runs/synth*/*.sh\n*/*.runs/synth*/*.tcl\n*/*.runs/synth*/*.bat\n*/*.runs/synth*/*.xdc\n!*/*.runs/synth*/*utilization*.rpt\n\n*.runs/synth*/*.xml\n*.runs/synth*/*.txt\n*.runs/synth*/*.sh\n*.runs/synth*/*.tcl\n*.runs/synth*/*.bat\n*.runs/synth*/*.xdc\n!*.runs/synth*/*utilization*.rpt\n\n### project impl\n*/*.runs/impl*/*.xml\n*/*.runs/impl*/*.html\n*/*.runs/impl*/*.txt\n*/*.runs/impl*/*.sh\n*/*.runs/impl*/*.tcl\n*/*.runs/impl*/*.bat\n!*/*.runs/impl*/*utilization*.rpt\n\n*.runs/impl*/*.xml\n*.runs/impl*/*.html\n*.runs/impl*/*.txt\n*.runs/impl*/*.sh\n*.runs/impl*/*.tcl\n*.runs/impl*/*.bat\n!*.runs/impl*/*utilization*.rpt\n\n### block design\n*/*/bd/*/hdl\n*/*/*/bd/*/hdl\n\n*/*/bd/*/*.xdc\n*/*/*/bd/*/*.xdc\n\n*/*/bd/*/ip/*/*.xdc\n*/*/*/bd/*/ip/*/*.xdc\n\n*/*/bd/*/ip/*/*/\n*/*/*/bd/*/ip/*/*/\n\n*/*/bd/*/ip/*/*.vhd\n*/*/*/bd/*/ip/*/*.vhd\n\n*/*/bd/*/ip/*/*.xml\n*/*/*/bd/*/ip/*/*.xml\n\n*.c\n*.h\n*.vho\n*.html\n*/*/bd/*/ip/*/*.tcl\n*/*/*/bd/*/ip/*/*.tcl\nhw_handoff\nipshared\n" }
	}

	fn file_name(self) -> &'static str {
		match self {
			#[cfg(feature = "community-alteryx")]
			Self::Alteryx => "Alteryx.gitignore",
			#[cfg(feature = "community-altium-designer")]
			Self::AltiumDesigner => "AltiumDesigner.gitignore",
			#[cfg(feature = "community-auto-it")]
			Self::AutoIt => "AutoIt.gitignore",
			#[cfg(feature = "community-automation-studio")]
			Self::AutomationStudio => "AutomationStudio.gitignore",
			#[cfg(feature = "community-aws-cdk")]
			Self::AwsCdk => "CDK.gitignore",
			#[cfg(feature = "community-aws-sam")]
			Self::AwsSam => "SAM.gitignore",
			#[cfg(feature = "community-b4x")]
			Self::B4x => "B4X.gitignore",
			#[cfg(feature = "community-bazel")]
			Self::Bazel => "Bazel.gitignore",
			#[cfg(feature = "community-beef")]
			Self::Beef => "Beef.gitignore",
			#[cfg(feature = "community-dot-net-core")]
			Self::DotNetCore => "core.gitignore",
			#[cfg(feature = "community-dot-net-infor-cms")]
			Self::DotNetInforCms => "InforCMS.gitignore",
			#[cfg(feature = "community-dot-net-kentico")]
			Self::DotNetKentico => "Kentico.gitignore",
			#[cfg(feature = "community-dot-net-umbraco")]
			Self::DotNetUmbraco => "Umbraco.gitignore",
			#[cfg(feature = "community-elixir-phoenix")]
			Self::ElixirPhoenix => "Phoenix.gitignore",
			#[cfg(feature = "community-embedded-atmel-studio")]
			Self::EmbeddedAtmelStudio => "AtmelStudio.gitignore",
			#[cfg(feature = "community-embedded-esp-idf")]
			Self::EmbeddedEspIdf => "esp-idf.gitignore",
			#[cfg(feature = "community-embedded-iar-ewarm")]
			Self::EmbeddedIarEwarm => "IAR_EWARM.gitignore",
			#[cfg(feature = "community-embedded-u-vision")]
			Self::EmbeddedUVision => "uVision.gitignore",
			#[cfg(feature = "community-exercism")]
			Self::Exercism => "Exercism.gitignore",
			#[cfg(feature = "community-gnome-gnome-shell-extension")]
			Self::GnomeGnomeShellExtension => "GNOMEShellExtension.gitignore",
			#[cfg(feature = "community-golang-go-allow-list")]
			Self::GolangGoAllowList => "Go.AllowList.gitignore",
			#[cfg(feature = "community-golang-hugo")]
			Self::GolangHugo => "Hugo.gitignore",
			#[cfg(feature = "community-gretl")]
			Self::Gretl => "Gretl.gitignore",
			#[cfg(feature = "community-hexo")]
			Self::Hexo => "Hexo.gitignore",
			#[cfg(feature = "community-java-j-boss4")]
			Self::JavaJBoss4 => "JBoss4.gitignore",
			#[cfg(feature = "community-java-j-boss6")]
			Self::JavaJBoss6 => "JBoss6.gitignore",
			#[cfg(feature = "community-java-script-cordova")]
			Self::JavaScriptCordova => "Cordova.gitignore",
			#[cfg(feature = "community-java-script-meteor")]
			Self::JavaScriptMeteor => "Meteor.gitignore",
			#[cfg(feature = "community-java-script-n-wjs")]
			Self::JavaScriptNWjs => "NWjs.gitignore",
			#[cfg(feature = "community-java-script-vue")]
			Self::JavaScriptVue => "Vue.gitignore",
			#[cfg(feature = "community-lens-studio")]
			Self::LensStudio => "LensStudio.gitignore",
			#[cfg(feature = "community-linux-snap")]
			Self::LinuxSnap => "Snap.gitignore",
			#[cfg(feature = "community-logtalk")]
			Self::Logtalk => "Logtalk.gitignore",
			#[cfg(feature = "community-move")]
			Self::Move => "Move.gitignore",
			#[cfg(feature = "community-nasa-specs-intact")]
			Self::NasaSpecsIntact => "NasaSpecsIntact.gitignore",
			#[cfg(feature = "community-nix")]
			Self::Nix => "Nix.gitignore",
			#[cfg(feature = "community-obsidian-notes-and-core-configuration")]
			Self::ObsidianNotesAndCoreConfiguration => "NotesAndCoreConfiguration.gitignore",
			#[cfg(feature = "community-obsidian-notes-and-extended-configuration")]
			Self::ObsidianNotesAndExtendedConfiguration => "NotesAndExtendedConfiguration.gitignore",
			#[cfg(feature = "community-obsidian-notes-only")]
			Self::ObsidianNotesOnly => "NotesOnly.gitignore",
			#[cfg(feature = "community-open-ssl")]
			Self::OpenSsl => "OpenSSL.gitignore",
			#[cfg(feature = "community-open-tofu")]
			Self::OpenTofu => "OpenTofu.gitignore",
			#[cfg(feature = "community-php-bitrix")]
			Self::PhpBitrix => "Bitrix.gitignore",
			#[cfg(feature = "community-php-code-sniffer")]
			Self::PhpCodeSniffer => "CodeSniffer.gitignore",
			#[cfg(feature = "community-php-drupal7")]
			Self::PhpDrupal7 => "Drupal7.gitignore",
			#[cfg(feature = "community-php-jigsaw")]
			Self::PhpJigsaw => "Jigsaw.gitignore",
			#[cfg(feature = "community-php-magento1")]
			Self::PhpMagento1 => "Magento1.gitignore",
			#[cfg(feature = "community-php-magento2")]
			Self::PhpMagento2 => "Magento2.gitignore",
			#[cfg(feature = "community-php-pimcore")]
			Self::PhpPimcore => "Pimcore.gitignore",
			#[cfg(feature = "community-php-think-php")]
			Self::PhpThinkPhp => "ThinkPHP.gitignore",
			#[cfg(feature = "community-puppet")]
			Self::Puppet => "Puppet.gitignore",
			#[cfg(feature = "community-python-jupyter-notebooks")]
			Self::PythonJupyterNotebooks => "JupyterNotebooks.gitignore",
			#[cfg(feature = "community-python-nikola")]
			Self::PythonNikola => "Nikola.gitignore",
			#[cfg(feature = "community-racket")]
			Self::Racket => "Racket.gitignore",
			#[cfg(feature = "community-red")]
			Self::Red => "Red.gitignore",
			#[cfg(feature = "community-ros2")]
			Self::Ros2 => "ROS2.gitignore",
			#[cfg(feature = "community-sp-fx")]
			Self::SpFx => "SPFx.gitignore",
			#[cfg(feature = "community-splunk")]
			Self::Splunk => "Splunk.gitignore",
			#[cfg(feature = "community-strapi")]
			Self::Strapi => "Strapi.gitignore",
			#[cfg(feature = "community-terragrunt")]
			Self::Terragrunt => "Terragrunt.gitignore",
			#[cfg(feature = "community-toit")]
			Self::Toit => "Toit.gitignore",
			#[cfg(feature = "community-ui-path")]
			Self::UiPath => "UiPath.gitignore",
			#[cfg(feature = "community-v")]
			Self::V => "V.gitignore",
			#[cfg(feature = "community-xilinx")]
			Self::Xilinx => "Xilinx.gitignore",
		}
	}

	fn file_path(self) -> &'static str {
		match self {
			#[cfg(feature = "community-alteryx")]
			Self::Alteryx => "community/Alteryx.gitignore",
			#[cfg(feature = "community-altium-designer")]
			Self::AltiumDesigner => "community/AltiumDesigner.gitignore",
			#[cfg(feature = "community-auto-it")]
			Self::AutoIt => "community/AutoIt.gitignore",
			#[cfg(feature = "community-automation-studio")]
			Self::AutomationStudio => "community/AutomationStudio.gitignore",
			#[cfg(feature = "community-aws-cdk")]
			Self::AwsCdk => "community/AWS/CDK.gitignore",
			#[cfg(feature = "community-aws-sam")]
			Self::AwsSam => "community/AWS/SAM.gitignore",
			#[cfg(feature = "community-b4x")]
			Self::B4x => "community/B4X.gitignore",
			#[cfg(feature = "community-bazel")]
			Self::Bazel => "community/Bazel.gitignore",
			#[cfg(feature = "community-beef")]
			Self::Beef => "community/Beef.gitignore",
			#[cfg(feature = "community-dot-net-core")]
			Self::DotNetCore => "community/DotNet/core.gitignore",
			#[cfg(feature = "community-dot-net-infor-cms")]
			Self::DotNetInforCms => "community/DotNet/InforCMS.gitignore",
			#[cfg(feature = "community-dot-net-kentico")]
			Self::DotNetKentico => "community/DotNet/Kentico.gitignore",
			#[cfg(feature = "community-dot-net-umbraco")]
			Self::DotNetUmbraco => "community/DotNet/Umbraco.gitignore",
			#[cfg(feature = "community-elixir-phoenix")]
			Self::ElixirPhoenix => "community/Elixir/Phoenix.gitignore",
			#[cfg(feature = "community-embedded-atmel-studio")]
			Self::EmbeddedAtmelStudio => "community/embedded/AtmelStudio.gitignore",
			#[cfg(feature = "community-embedded-esp-idf")]
			Self::EmbeddedEspIdf => "community/embedded/esp-idf.gitignore",
			#[cfg(feature = "community-embedded-iar-ewarm")]
			Self::EmbeddedIarEwarm => "community/embedded/IAR_EWARM.gitignore",
			#[cfg(feature = "community-embedded-u-vision")]
			Self::EmbeddedUVision => "community/embedded/uVision.gitignore",
			#[cfg(feature = "community-exercism")]
			Self::Exercism => "community/Exercism.gitignore",
			#[cfg(feature = "community-gnome-gnome-shell-extension")]
			Self::GnomeGnomeShellExtension => "community/GNOME/GNOMEShellExtension.gitignore",
			#[cfg(feature = "community-golang-go-allow-list")]
			Self::GolangGoAllowList => "community/Golang/Go.AllowList.gitignore",
			#[cfg(feature = "community-golang-hugo")]
			Self::GolangHugo => "community/Golang/Hugo.gitignore",
			#[cfg(feature = "community-gretl")]
			Self::Gretl => "community/Gretl.gitignore",
			#[cfg(feature = "community-hexo")]
			Self::Hexo => "community/Hexo.gitignore",
			#[cfg(feature = "community-java-j-boss4")]
			Self::JavaJBoss4 => "community/Java/JBoss4.gitignore",
			#[cfg(feature = "community-java-j-boss6")]
			Self::JavaJBoss6 => "community/Java/JBoss6.gitignore",
			#[cfg(feature = "community-java-script-cordova")]
			Self::JavaScriptCordova => "community/JavaScript/Cordova.gitignore",
			#[cfg(feature = "community-java-script-meteor")]
			Self::JavaScriptMeteor => "community/JavaScript/Meteor.gitignore",
			#[cfg(feature = "community-java-script-n-wjs")]
			Self::JavaScriptNWjs => "community/JavaScript/NWjs.gitignore",
			#[cfg(feature = "community-java-script-vue")]
			Self::JavaScriptVue => "community/JavaScript/Vue.gitignore",
			#[cfg(feature = "community-lens-studio")]
			Self::LensStudio => "community/LensStudio.gitignore",
			#[cfg(feature = "community-linux-snap")]
			Self::LinuxSnap => "community/Linux/Snap.gitignore",
			#[cfg(feature = "community-logtalk")]
			Self::Logtalk => "community/Logtalk.gitignore",
			#[cfg(feature = "community-move")]
			Self::Move => "community/Move.gitignore",
			#[cfg(feature = "community-nasa-specs-intact")]
			Self::NasaSpecsIntact => "community/NasaSpecsIntact.gitignore",
			#[cfg(feature = "community-nix")]
			Self::Nix => "community/Nix.gitignore",
			#[cfg(feature = "community-obsidian-notes-and-core-configuration")]
			Self::ObsidianNotesAndCoreConfiguration => {
				"community/Obsidian/NotesAndCoreConfiguration.gitignore"
			}
			#[cfg(feature = "community-obsidian-notes-and-extended-configuration")]
			Self::ObsidianNotesAndExtendedConfiguration => {
				"community/Obsidian/NotesAndExtendedConfiguration.gitignore"
			}
			#[cfg(feature = "community-obsidian-notes-only")]
			Self::ObsidianNotesOnly => "community/Obsidian/NotesOnly.gitignore",
			#[cfg(feature = "community-open-ssl")]
			Self::OpenSsl => "community/OpenSSL.gitignore",
			#[cfg(feature = "community-open-tofu")]
			Self::OpenTofu => "community/OpenTofu.gitignore",
			#[cfg(feature = "community-php-bitrix")]
			Self::PhpBitrix => "community/PHP/Bitrix.gitignore",
			#[cfg(feature = "community-php-code-sniffer")]
			Self::PhpCodeSniffer => "community/PHP/CodeSniffer.gitignore",
			#[cfg(feature = "community-php-drupal7")]
			Self::PhpDrupal7 => "community/PHP/Drupal7.gitignore",
			#[cfg(feature = "community-php-jigsaw")]
			Self::PhpJigsaw => "community/PHP/Jigsaw.gitignore",
			#[cfg(feature = "community-php-magento1")]
			Self::PhpMagento1 => "community/PHP/Magento1.gitignore",
			#[cfg(feature = "community-php-magento2")]
			Self::PhpMagento2 => "community/PHP/Magento2.gitignore",
			#[cfg(feature = "community-php-pimcore")]
			Self::PhpPimcore => "community/PHP/Pimcore.gitignore",
			#[cfg(feature = "community-php-think-php")]
			Self::PhpThinkPhp => "community/PHP/ThinkPHP.gitignore",
			#[cfg(feature = "community-puppet")]
			Self::Puppet => "community/Puppet.gitignore",
			#[cfg(feature = "community-python-jupyter-notebooks")]
			Self::PythonJupyterNotebooks => "community/Python/JupyterNotebooks.gitignore",
			#[cfg(feature = "community-python-nikola")]
			Self::PythonNikola => "community/Python/Nikola.gitignore",
			#[cfg(feature = "community-racket")]
			Self::Racket => "community/Racket.gitignore",
			#[cfg(feature = "community-red")]
			Self::Red => "community/Red.gitignore",
			#[cfg(feature = "community-ros2")]
			Self::Ros2 => "community/ROS2.gitignore",
			#[cfg(feature = "community-sp-fx")]
			Self::SpFx => "community/SPFx.gitignore",
			#[cfg(feature = "community-splunk")]
			Self::Splunk => "community/Splunk.gitignore",
			#[cfg(feature = "community-strapi")]
			Self::Strapi => "community/Strapi.gitignore",
			#[cfg(feature = "community-terragrunt")]
			Self::Terragrunt => "community/Terragrunt.gitignore",
			#[cfg(feature = "community-toit")]
			Self::Toit => "community/Toit.gitignore",
			#[cfg(feature = "community-ui-path")]
			Self::UiPath => "community/UiPath.gitignore",
			#[cfg(feature = "community-v")]
			Self::V => "community/V.gitignore",
			#[cfg(feature = "community-xilinx")]
			Self::Xilinx => "community/Xilinx.gitignore",
		}
	}

	#[cfg(feature = "std")]
	fn list() -> Vec<&'static str> {
		#[allow(unused_mut)]
		let mut list = Vec::with_capacity(63);
		#[cfg(feature = "community-alteryx")]
		list.push("Alteryx");
		#[cfg(feature = "community-altium-designer")]
		list.push("AltiumDesigner");
		#[cfg(feature = "community-auto-it")]
		list.push("AutoIt");
		#[cfg(feature = "community-automation-studio")]
		list.push("AutomationStudio");
		#[cfg(feature = "community-aws-cdk")]
		list.push("AwsCdk");
		#[cfg(feature = "community-aws-sam")]
		list.push("AwsSam");
		#[cfg(feature = "community-b4x")]
		list.push("B4x");
		#[cfg(feature = "community-bazel")]
		list.push("Bazel");
		#[cfg(feature = "community-beef")]
		list.push("Beef");
		#[cfg(feature = "community-dot-net-core")]
		list.push("DotNetCore");
		#[cfg(feature = "community-dot-net-infor-cms")]
		list.push("DotNetInforCms");
		#[cfg(feature = "community-dot-net-kentico")]
		list.push("DotNetKentico");
		#[cfg(feature = "community-dot-net-umbraco")]
		list.push("DotNetUmbraco");
		#[cfg(feature = "community-elixir-phoenix")]
		list.push("ElixirPhoenix");
		#[cfg(feature = "community-embedded-atmel-studio")]
		list.push("EmbeddedAtmelStudio");
		#[cfg(feature = "community-embedded-esp-idf")]
		list.push("EmbeddedEspIdf");
		#[cfg(feature = "community-embedded-iar-ewarm")]
		list.push("EmbeddedIarEwarm");
		#[cfg(feature = "community-embedded-u-vision")]
		list.push("EmbeddedUVision");
		#[cfg(feature = "community-exercism")]
		list.push("Exercism");
		#[cfg(feature = "community-gnome-gnome-shell-extension")]
		list.push("GnomeGnomeShellExtension");
		#[cfg(feature = "community-golang-go-allow-list")]
		list.push("GolangGoAllowList");
		#[cfg(feature = "community-golang-hugo")]
		list.push("GolangHugo");
		#[cfg(feature = "community-gretl")]
		list.push("Gretl");
		#[cfg(feature = "community-hexo")]
		list.push("Hexo");
		#[cfg(feature = "community-java-j-boss4")]
		list.push("JavaJBoss4");
		#[cfg(feature = "community-java-j-boss6")]
		list.push("JavaJBoss6");
		#[cfg(feature = "community-java-script-cordova")]
		list.push("JavaScriptCordova");
		#[cfg(feature = "community-java-script-meteor")]
		list.push("JavaScriptMeteor");
		#[cfg(feature = "community-java-script-n-wjs")]
		list.push("JavaScriptNWjs");
		#[cfg(feature = "community-java-script-vue")]
		list.push("JavaScriptVue");
		#[cfg(feature = "community-lens-studio")]
		list.push("LensStudio");
		#[cfg(feature = "community-linux-snap")]
		list.push("LinuxSnap");
		#[cfg(feature = "community-logtalk")]
		list.push("Logtalk");
		#[cfg(feature = "community-move")]
		list.push("Move");
		#[cfg(feature = "community-nasa-specs-intact")]
		list.push("NasaSpecsIntact");
		#[cfg(feature = "community-nix")]
		list.push("Nix");
		#[cfg(feature = "community-obsidian-notes-and-core-configuration")]
		list.push("ObsidianNotesAndCoreConfiguration");
		#[cfg(feature = "community-obsidian-notes-and-extended-configuration")]
		list.push("ObsidianNotesAndExtendedConfiguration");
		#[cfg(feature = "community-obsidian-notes-only")]
		list.push("ObsidianNotesOnly");
		#[cfg(feature = "community-open-ssl")]
		list.push("OpenSsl");
		#[cfg(feature = "community-open-tofu")]
		list.push("OpenTofu");
		#[cfg(feature = "community-php-bitrix")]
		list.push("PhpBitrix");
		#[cfg(feature = "community-php-code-sniffer")]
		list.push("PhpCodeSniffer");
		#[cfg(feature = "community-php-drupal7")]
		list.push("PhpDrupal7");
		#[cfg(feature = "community-php-jigsaw")]
		list.push("PhpJigsaw");
		#[cfg(feature = "community-php-magento1")]
		list.push("PhpMagento1");
		#[cfg(feature = "community-php-magento2")]
		list.push("PhpMagento2");
		#[cfg(feature = "community-php-pimcore")]
		list.push("PhpPimcore");
		#[cfg(feature = "community-php-think-php")]
		list.push("PhpThinkPhp");
		#[cfg(feature = "community-puppet")]
		list.push("Puppet");
		#[cfg(feature = "community-python-jupyter-notebooks")]
		list.push("PythonJupyterNotebooks");
		#[cfg(feature = "community-python-nikola")]
		list.push("PythonNikola");
		#[cfg(feature = "community-racket")]
		list.push("Racket");
		#[cfg(feature = "community-red")]
		list.push("Red");
		#[cfg(feature = "community-ros2")]
		list.push("Ros2");
		#[cfg(feature = "community-sp-fx")]
		list.push("SpFx");
		#[cfg(feature = "community-splunk")]
		list.push("Splunk");
		#[cfg(feature = "community-strapi")]
		list.push("Strapi");
		#[cfg(feature = "community-terragrunt")]
		list.push("Terragrunt");
		#[cfg(feature = "community-toit")]
		list.push("Toit");
		#[cfg(feature = "community-ui-path")]
		list.push("UiPath");
		#[cfg(feature = "community-v")]
		list.push("V");
		#[cfg(feature = "community-xilinx")]
		list.push("Xilinx");
		list
	}
}

#[cfg(all(feature = "std", not(feature = "no-contents")))]
impl std::fmt::Display for Community {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.contents())
	}
}
