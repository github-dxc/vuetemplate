<!DOCTYPE html>
<html>
<head>
	<meta http-equiv="Content-type" content="text/html; charset=utf-8" />
	<meta name="robots" content="noindex,follow" />
	<title>提交问题 - MantisBT</title>
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/default.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/status_config.php?cache_key=a771ef2ee247e9ccb4bf6207a8bffcc3" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/dropzone-5.5.0.min.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/bootstrap-3.4.1.min.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/font-awesome-4.6.3.min.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/fonts.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/bootstrap-datetimepicker-4.17.47.min.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/ace.min.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/ace-mantis.css" />
	<link rel="stylesheet" type="text/css" href="http://localhost:8989/css/ace-skins.min.css" />

	<link rel="shortcut icon" href="/images/favicon.ico" type="image/x-icon" />
	<link rel="search" type="application/opensearchdescription+xml" title="MantisBT：全文搜索" href="http://localhost:8989/browser_search_plugin.php?type=text"/>
	<link rel="search" type="application/opensearchdescription+xml" title="MantisBT：按问题Id搜索" href="http://localhost:8989/browser_search_plugin.php?type=id"/>
	<script type="text/javascript" src="/javascript_config.php?cache_key=a771ef2ee247e9ccb4bf6207a8bffcc3"></script>
	<script type="text/javascript" src="/javascript_translations.php?cache_key=d7ad9d9f5efebc5d53d636970bfb5082"></script>
	<script type="text/javascript" src="/js/jquery-2.2.4.min.js"></script>
	<script type="text/javascript" src="/js/dropzone-5.5.0.min.js"></script>
	<script type="text/javascript" src="/js/common.js"></script>
</head>
<body class="skin-3">
<style>
* { font-family: "Open Sans"; } 
h1, h2, h3, h4, h5 { font-family: "Open Sans"; } 
</style>
<div id="navbar" class="navbar navbar-default navbar-collapse navbar-fixed-top noprint"><div id="navbar-container" class="navbar-container"><button id="menu-toggler" type="button" class="navbar-toggle menu-toggler pull-left hidden-lg hidden-md" data-target="#sidebar"><span class="sr-only">Toggle sidebar</span><span class="icon-bar"></span><span class="icon-bar"></span><span class="icon-bar"></span></button><div class="navbar-header"><a href="/my_view_page.php" class="navbar-brand"><span class="smaller-75"> MantisBT </span></a><button type="button" class="navbar-toggle navbar-toggle collapsed pull-right hidden-sm hidden-md hidden-lg" data-toggle="collapse" data-target=".navbar-buttons,.navbar-menu"><span class="sr-only">Toggle user menu</span><i class="ace-icon fa fa-user fa-2x white"></i> </button></div><div class="navbar-buttons navbar-header navbar-collapse collapse"><ul class="nav ace-nav"><li class="hidden-sm hidden-xs"><div class="btn-group btn-corner padding-right-8 padding-left-8"><a class="btn btn-primary btn-sm" href="manage_user_create_page.php"><i class="fa fa-user-plus"></i> 邀请用户</a></div></li><li class="grey" id="dropdown_projects_menu">
<a data-toggle="dropdown" href="#" class="dropdown-toggle">
&#160;2&#160;
 <i class="ace-icon fa fa-angle-down bigger-110"></i>
</a>
<ul id="projects-list" class=" dropdown-menu dropdown-menu-right dropdown-yellow dropdown-caret dropdown-close">
<li><div class="projects-searchbox"><input class="search form-control input-md" placeholder="搜索" /></div></li><li class="divider"></li>
<li><div class="scrollable-menu"><ul class="list dropdown-yellow no-margin"><li><a href="/set_project.php?project_id=0">所有项目 </a></li>
<li class="divider"></li>
<li class="active"><a href="/set_project.php?project_id=3" class="project-link"> 2 </a></li>
<li><a href="/set_project.php?project_id=1" class="project-link"> 测试项目 </a></li>
<li><a href="/set_project.php?project_id=1;2" class="project-link"> &#160;&#160;&#160;&#160;1</a></li>
<li><a href="/set_project.php?project_id=1;4" class="project-link"> &#160;&#160;&#160;&#160;cs1</a></li>
</ul></div></li></ul>
</li>
<li class="grey"><a data-toggle="dropdown" href="#" class="dropdown-toggle"><i class="ace-icon fa fa-user fa-2x white"></i> <span class="user-info">administrator</span><i class="ace-icon fa fa-angle-down"></i></a><ul class="user-menu dropdown-menu dropdown-menu-right dropdown-yellow dropdown-caret dropdown-close"><li><a href="/account_page.php"><i class="ace-icon fa fa-user"> </i> 个人资料</a></li><li><a href="http://localhost:8989/issues_rss.php?username=administrator&amp;key=93tkWJdEZkuyfh0FAVGxjL7_J9abJGLaq_fYy7PFhTWFqOxNnTJgB9SEVEbos1kJKhDa8-Gga2wQibZbbIdm&amp;project_id=3"><i class="ace-icon fa fa-rss-square orange"> </i> RSS</a></li><li class="divider"></li><li><a href="/logout_page.php"><i class="ace-icon fa fa-sign-out"> </i> 注销</a></li></ul></li></ul></div></div></div><div class="main-container" id="main-container">
<div id="sidebar" class="sidebar sidebar-fixed responsive compact "><ul class="nav nav-list"><li>
<a href="/my_view_page.php">
<i class="menu-icon fa fa-dashboard"></i> 
<span class="menu-text"> 我的视图 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/view_all_bug_page.php">
<i class="menu-icon fa fa-list-alt"></i> 
<span class="menu-text"> 查看问题 </span>
</a>
<b class="arrow"></b>
</li>
<li class="active">
<a href="/bug_report_page.php">
<i class="menu-icon fa fa-edit"></i> 
<span class="menu-text"> 提交问题 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/changelog_page.php">
<i class="menu-icon fa fa-retweet"></i> 
<span class="menu-text"> 变更日志 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/roadmap_page.php">
<i class="menu-icon fa fa-road"></i> 
<span class="menu-text"> 路线图 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/summary_page.php">
<i class="menu-icon fa fa-bar-chart-o"></i> 
<span class="menu-text"> 摘要 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/manage_overview_page.php">
<i class="menu-icon fa fa-gears"></i> 
<span class="menu-text"> 管理 </span>
</a>
<b class="arrow"></b>
</li>
</ul><div id="sidebar-btn" class="sidebar-toggle sidebar-collapse"><i data-icon2="ace-icon fa fa-angle-double-right" data-icon1="ace-icon fa fa-angle-double-left"
		class="ace-icon fa fa-angle-double-left"></i></div></div><div class="main-content">
<div id="breadcrumbs" class="breadcrumbs noprint">
<ul class="breadcrumb">
  <li><i class="fa fa-user home-icon active"></i>  <a href="/account_page.php">administrator ( 邓祥成 ) </a>
  <span class="label hidden-xs label-default arrowed">管理员</span></li>
</ul>
<div id="nav-search" class="nav-search"><form class="form-search" method="post" action="/jump_to_bug.php"><span class="input-icon"><input type="text" name="bug_id" autocomplete="off" class="nav-search-input" placeholder="问题 #"><i class="ace-icon fa fa-search nav-search-icon"></i></span></form></div>
</div>
  <div class="page-content">
<div class="row">
<div class="col-md-12 col-xs-12">
<form id="report_bug_form"
	method="post" enctype="multipart/form-data"	action="bug_report.php?posted=1">
<input type="hidden" name="bug_report_token" value="20250720ZEhwUtrkkQURsr-73w_GuqR8xU8ee_Fp"/><input type="hidden" name="m_id" value="0" />
<input type="hidden" name="project_id" value="3" />
<div class="widget-box widget-color-blue2">
	<div class="widget-header widget-header-small">
		<h4 class="widget-title lighter">
				<i class="ace-icon fa fa-edit"></i>
				输入问题详情		</h4>
	</div>
<div class="widget-body dz-clickable">
<div class="widget-main no-padding">
<div class="table-responsive">
<table class="table table-bordered table-condensed">
	<tr>
		<th class="category" width="30%">
			<span class="required">*</span> <label for="category_id">分类</label>		</th>
		<td width="70%">
						<select tabindex="1" id="category_id" name="category_id" class="autofocus input-sm">
				<option value="1" selected="selected">[所有项目] General</option>			</select>
		</td>
	</tr>

	<tr>
		<th class="category">
			<label for="reproducibility">出现频率</label>
		</th>
		<td>
			<select tabindex="2" id="reproducibility" name="reproducibility" class="input-sm">
				<option value="10">总是</option><option value="30">有时</option><option value="50">随机</option><option value="70" selected="selected">没有试验</option><option value="90">无法重现</option><option value="100">不适用</option>			</select>
		</td>
	</tr>
	<tr>
		<th class="category">
			<label for="severity">严重性</label>
		</th>
		<td>
			<select tabindex="3" id="severity" name="severity" class="input-sm">
				<option value="10">新功能</option><option value="20">细节</option><option value="30">文字</option><option value="40">小调整</option><option value="50" selected="selected">小错误</option><option value="60">很严重</option><option value="70">崩溃</option><option value="80">宕机</option>			</select>
		</td>
	</tr>
	<tr>
		<th class="category">
			<label for="priority">优先级</label>
		</th>
		<td>
			<select tabindex="4" id="priority" name="priority" class="input-sm">
				<option value="10">无</option><option value="20">低</option><option value="30" selected="selected">中</option><option value="40">高</option><option value="50">紧急</option><option value="60">非常紧急</option>			</select>
		</td>
	</tr>
	<tr>
		<th class="category">
			<label for="profile_id">选择平台配置</label>
		</th>
		<td>
						
<div id="profile_open" class=" collapse-closed noprint"><a id="profile_open_link" class="collapse-link noprint"><i class="fa fa-minus-square-o" title="-"></i></a>			或填写			<table class="table-bordered table-condensed">
				<tr>
					<th class="category" width="30%">
						<label for="platform">平台</label>
					</th>
					<td>
						<input type="text" id="platform" name="platform" class="typeahead input-sm" autocomplete = "off" size="32" maxlength="32" tabindex="5" value="" />					</td>
				</tr>
				<tr>
					<th class="category">
						<label for="os">操作系统</label>
					</th>
					<td>
						<input type="text" id="os" name="os" class="typeahead input-sm" autocomplete = "off" size="32" maxlength="32" tabindex="6" value="" />					</td>
				</tr>
				<tr>
					<th class="category">
						<label for="os_build">操作系统版本</label>
					</th>
					<td>
						<input type="text" id="os_build" name="os_build" class="typeahead input-sm" autocomplete = "off" size="16" maxlength="16" tabindex="7" value="" />					</td>
				</tr>
			</table>
			</div>
<div id="profile_closed" class="collapse-section-closed collapse-open"><a id="profile_closed_link" class="collapse-link noprint"><i class="fa fa-plus-square-o" title="+"></i></a>			或填写			</div>		</td>
	</tr>

	<tr>
		<th class="category">
			<label for="handler_id">分派给</label>
		</th>
		<td>
			<select tabindex="8" id="handler_id" name="handler_id" class="input-sm">
				<option value="0" selected="selected"></option>
				<option value="1" >administrator</option>			</select>
		</td>
	</tr>




	<tr>
		<th class="category">
			<span class="required">*</span><label for="summary">摘要</label>
		</th>
		<td>
			<input tabindex="9" type="text" id="summary" name="summary" size="105" maxlength="128" value="" required />
		</td>
	</tr>
	<tr>
		<th class="category">
			<span class="required">*</span><label for="description">描述</label>
		</th>
		<td>
						<textarea class="form-control" tabindex="10" id="description" name="description" cols="80" rows="10" required>
</textarea>
		</td>
	</tr>

		<tr>
			<th class="category">
				<label for="steps_to_reproduce">问题重现步骤</label>
			</th>
			<td>
								<textarea class="form-control" tabindex="11" id="steps_to_reproduce" name="steps_to_reproduce" cols="80" rows="10">
</textarea>
			</td>
		</tr>

	<tr>
		<th class="category">
			<label for="additional_info">附注</label>
		</th>
		<td>
						<textarea class="form-control" tabindex="12" id="additional_info" name="additional_info" cols="80" rows="10">
</textarea>
		</td>
	</tr>
	<tr>
		<th class="category">
			<label for="attach_tag">添加标签</label>
		</th>
		<td>
				<label class="inline small">(用","分割)</label>
	<input type="hidden" id="tag_separator" value="," />
	<input type="text" name="tag_string" id="tag_string" class="input-sm" size="40" value="" />
	<select class="input-sm" tabindex="13" name="tag_select" id="tag_select" class="input-sm">
		<option value="0">现有标签</option>	</select>
		</td>
	</tr>
	<tr>
		<th class="category">
			<label for="ufile[]">上传文件</label>
			<br />
			<span class="small" title="2,097,152 字节">最大尺寸：&#32;2,097 KB</span>		</th>
		<td>
				<div id="dropzone-preview-template" class="hidden">
		<div class="dz-preview dz-file-preview">
			<div class="dz-filename"><span data-dz-name></span></div>
			<div><img data-dz-thumbnail /></div>
			<div class="dz-size" data-dz-size></div>
			<div class="progress progress-small progress-striped active">
				<div class="progress-bar progress-bar-success" data-dz-uploadprogress></div>
			</div>
			<div class="dz-success-mark"><span></span></div>
			<div class="dz-error-mark"><span></span></div>
			<div class="dz-error-message"><span data-dz-errormessage></span></div>
		</div>
	</div>
				<input type="hidden" name="max_file_size" value="2097152" />
			<div class="dropzone center" data-force-fallback="false"
	data-max-filesize-bytes="2097152"
	data-accepted-files=""
	data-default-message="将文件放在这里上传（或点击）"
	data-fallback-message="您的浏览器不支持拖放文件上传。"
	data-fallback-text=""
	data-file-too-big="文件太大（{{filesize}}MiB）。最大文件大小：{{maxFilesize}}MiB。"
	data-invalid-file-type="您不能上传此类型的文件。"
	data-response-error="服务器响应了{{statusCode}}代码。"
	data-cancel-upload="取消上传"
	data-cancel-upload-confirmation="您确定要取消此次上传么？"
	data-remove-file="移除文件"
	data-remove-file-confirmation=""
	data-max-files-exceeded="您不能上传更多文件。"
	data-dropzone-not-supported="Dropzone.js不支持旧浏览器！"	data-dropzone_multiple_files_too_big="下列文件:{{files}}超出了最大文件限制 ({{maxFilesize}} MiB)">
				<i class="upload-icon ace-icon fa fa-cloud-upload blue fa-3x"></i><br>
				<span class="bigger-150 grey">将文件放在这里上传（或点击）</span>
				<div id="dropzone-previews-box" class="dropzone-previews dz-max-files-reached"></div>
			</div>
			<div class="fallback">
				<div class="dz-message" data-dz-message></div>
				<input tabindex="14" id="ufile[]" name="ufile[]" type="file" size="60" />
			</div>
		</td>
	</tr>

	<tr>
		<th class="category">
			查看权限		</th>
		<td>
			<label>
				<input tabindex="15" type="radio" class="ace" name="view_state" value="10"  checked="checked" />
				<span class="lbl padding-6">公开</span>
			</label>
			&#160;&#160;&#160;&#160;
			<label>
				<input tabindex="16" type="radio" class="ace" name="view_state" value="50"  />
				<span class="lbl padding-6">私有</span>
			</label>
		</td>
	</tr>
	<tr>
		<th class="category">
			继续报告		</th>
		<td>
			<label>
				<input tabindex="17" type="checkbox" class="ace" id="report_stay" name="report_stay"  />
				<span class="lbl padding-6">报告更多的问题</span>
			</label>
		</td>
	</tr>
</table>
</div>
</div>
<div class="widget-toolbox padding-8 clearfix">
	<span class="required pull-right"> * 必填</span>
	<input tabindex="18" type="submit" class="btn btn-primary btn-white btn-round" value="提交问题" />
</div>
</div>
</div>
</form>
</div>
</div>
</div>
</div>
<div class="clearfix"></div>
<div class="space-20"></div>
<div class="footer noprint">
<div class="footer-inner">
<div class="footer-content">
<div class="col-md-6 col-xs-12 no-padding">
<address>
<strong>Powered by <a href="https://www.mantisbt.org" title="bug tracking software">MantisBT </a></strong> <br>
<small>Copyright &copy; 2000 - 2025 MantisBT Team</small><br><small>联系<a href="mailto:webmaster@example.com" title="Contact the webmaster via e-mail.">管理员</a>获取援助</small><br>
</address>
</div>
<div class="col-md-6 col-xs-12">
<div class="pull-right" id="powered-by-mantisbt-logo">
<a href="https://www.mantisbt.org" title="Mantis Bug Tracker: a free and open source web based bug tracking system."><img src="/images/mantis_logo.png" width="102" height="35" alt="Powered by Mantis Bug Tracker: a free and open source web based bug tracking system." /></a>
</div>
</div>
</div>
</div>
</div>
<a class="btn-scroll-up btn btn-sm btn-inverse display" id="btn-scroll-up" href="#">
<i class="ace-icon fa fa-angle-double-up icon-only bigger-110"></i>
</a>
</div>
	<script type="text/javascript" src="/js/bootstrap-3.4.1.min.js"></script>
	<script type="text/javascript" src="/js/moment-with-locales-2.15.2.min.js"></script>
	<script type="text/javascript" src="/js/bootstrap-datetimepicker-4.17.47.min.js"></script>
	<script type="text/javascript" src="/js/typeahead.jquery-1.1.1.min.js"></script>
	<script type="text/javascript" src="/js/list-1.5.0.min.js"></script>
	<script type="text/javascript" src="/js/ace.min.js"></script>
</body>
</html>
