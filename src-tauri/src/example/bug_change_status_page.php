<!DOCTYPE html>
<html>
<head>
	<meta http-equiv="Content-type" content="text/html; charset=utf-8" />
	<title>0002491: 【运营管理-用户统计】今日累计充值用户数量错误 - MantisBT</title>
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/default.css?cache_key=991bb728eea2923f36cfa9c64025e9ae" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/status_config.php?cache_key=990a348b488b9c163bcdc2bc816bb905" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/dropzone-5.5.0.min.css" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/bootstrap-3.4.1.min.css" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/font-awesome-4.7.0.min.css" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/fonts.css?cache_key=991bb728eea2923f36cfa9c64025e9ae" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/bootstrap-datetimepicker-4.17.47.min.css" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/ace.min.css?cache_key=991bb728eea2923f36cfa9c64025e9ae" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/ace-mantis.css?cache_key=991bb728eea2923f36cfa9c64025e9ae" />
	<link rel="stylesheet" type="text/css" href="http://bug.test.com/css/ace-skins.min.css?cache_key=991bb728eea2923f36cfa9c64025e9ae" />

	<link rel="shortcut icon" href="/images/favicon.ico" type="image/x-icon" />
	<link rel="search" type="application/opensearchdescription+xml" title="MantisBT：全文搜索" href="http://bug.test.com/browser_search_plugin.php?type=text"/>
	<link rel="search" type="application/opensearchdescription+xml" title="MantisBT：按问题Id搜索" href="http://bug.test.com/browser_search_plugin.php?type=id"/>
	<script type="text/javascript" src="/javascript_config.php?cache_key=990a348b488b9c163bcdc2bc816bb905"></script>
	<script type="text/javascript" src="/javascript_translations.php?cache_key=aaa06b70c166522c6368bc2bbf601c96"></script>
	<script type="text/javascript" src="/js/jquery-2.2.4.min.js"></script>
	<script type="text/javascript" src="/js/dropzone-5.5.0.min.js"></script>
	<script type="text/javascript" src="/js/common.js"></script>
</head>
<body class="skin-3">
<style>
* { font-family: "Open Sans"; } 
h1, h2, h3, h4, h5 { font-family: "Open Sans"; } 
</style>
<div id="navbar" class="navbar navbar-default navbar-collapse navbar-fixed-top noprint"><div id="navbar-container" class="navbar-container"><button id="menu-toggler" type="button" class="navbar-toggle menu-toggler pull-left hidden-lg hidden-md" data-target="#sidebar"><span class="sr-only">Toggle sidebar</span><span class="icon-bar"></span><span class="icon-bar"></span><span class="icon-bar"></span></button><div class="navbar-header"><a href="/my_view_page.php" class="navbar-brand"><span class="smaller-75"> MantisBT </span></a><button type="button" class="navbar-toggle navbar-toggle collapsed pull-right hidden-sm hidden-md hidden-lg" data-toggle="collapse" data-target=".navbar-buttons,.navbar-menu"><span class="sr-only">Toggle user menu</span><i class="fa fa-user ace-icon fa-2x white" ></i> </button></div><div class="navbar-buttons navbar-header navbar-collapse collapse"><ul class="nav ace-nav"><li class="hidden-sm hidden-xs"><div class="btn-group btn-corner padding-right-8 padding-left-8"><a class="btn btn-primary btn-sm" href="bug_report_page.php"><i class="fa fa-edit " ></i> 提交问题</a></div></li>
<li class="grey" id="dropdown_projects_menu">
<a data-toggle="dropdown" href="#" class="dropdown-toggle">
&#160;云计算海外版-管理端&#160;
<i class="fa fa-angle-down ace-icon bigger-110" ></i>
</a>
<ul id="projects-list" class=" dropdown-menu dropdown-menu-right dropdown-yellow dropdown-caret dropdown-close">
<li><div class="projects-searchbox"><input class="search form-control input-md" placeholder="搜索" /></div></li>
<li class="divider"></li>
<li><div class="scrollable-menu">
<ul class="list dropdown-yellow no-margin">
<li><a class="project-link" href="/set_project.php?project_id=0">所有项目</a></li>
<li class="divider"></li>
<li><a class="project-link" href="/set_project.php?project_id=22">EasyLink</a></li>
<li><a class="project-link" href="/set_project.php?project_id=22;24">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;EasyLink-用户端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=22;23">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;EasyLink-管理端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=9">云计算</a></li>
<li><a class="project-link" href="/set_project.php?project_id=9;11">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云计算-用户端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=9;10">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云计算-管理端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=9;21">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云计算-线上问题</a></li>
<li><a class="project-link" href="/set_project.php?project_id=26">云计算海外版</a></li>
<li><a class="project-link" href="/set_project.php?project_id=26;28">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云计算海外版-用户端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=26;27">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云计算海外版-管理端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=13">云转码</a></li>
<li><a class="project-link" href="/set_project.php?project_id=13;16">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云转码-用户端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=13;17">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;云转码-管理端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=12">证书系统</a></li>
<li><a class="project-link" href="/set_project.php?project_id=25">资源管理系统</a></li>
<li><a class="project-link" href="/set_project.php?project_id=18">钱包</a></li>
<li><a class="project-link" href="/set_project.php?project_id=18;20">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;钱包-用户端</a></li>
<li><a class="project-link" href="/set_project.php?project_id=18;19">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;钱包-管理端</a></li>
</ul>
</div>
</li>
</ul>
</li>
<li class="grey"><a data-toggle="dropdown" href="#" class="dropdown-toggle"><i class="fa fa-user ace-icon fa-2x white" ></i> <span class="user-info">dengxiangcheng</span><i class="fa fa-angle-down ace-icon" ></i></a><ul class="user-menu dropdown-menu dropdown-menu-right dropdown-yellow dropdown-caret dropdown-close"><li><a href="/account_page.php"><i class="fa fa-user ace-icon" ></i> 个人资料</a></li><li><a href="http://bug.test.com/issues_rss.php?username=dengxiangcheng&amp;key=8V1b0OxbiMncNLPZv5P0Z5CVGjKQUVdC7hpCwu-DrtDl3DLXctDBBjdpm70Sc1te1IW2cR_tm1rabvNNNLRf&amp;project_id=27"><i class="fa fa-rss-square orange ace-icon" ></i> RSS</a></li><li class="divider"></li><li><a href="/logout_page.php"><i class="fa fa-sign-out ace-icon" ></i> 注销</a></li></ul></li></ul></div></div></div><div class="main-container" id="main-container">
<div id="sidebar" class="sidebar sidebar-fixed responsive compact "><ul class="nav nav-list"><li>
<a href="/my_view_page.php">
<i class="fa fa-dashboard menu-icon" ></i>
<span class="menu-text"> 我的视图 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/view_all_bug_page.php">
<i class="fa fa-list-alt menu-icon" ></i>
<span class="menu-text"> 查看问题 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/bug_report_page.php">
<i class="fa fa-edit menu-icon" ></i>
<span class="menu-text"> 提交问题 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/changelog_page.php">
<i class="fa fa-retweet menu-icon" ></i>
<span class="menu-text"> 变更日志 </span>
</a>
<b class="arrow"></b>
</li>
<li>
<a href="/roadmap_page.php">
<i class="fa fa-road menu-icon" ></i>
<span class="menu-text"> 路线图 </span>
</a>
<b class="arrow"></b>
</li>
</ul><div id="sidebar-btn" class="sidebar-toggle sidebar-collapse"><i data-icon2="ace-icon fa fa-angle-double-right" data-icon1="ace-icon fa fa-angle-double-left"
		class="ace-icon fa fa-angle-double-left"></i></div></div><div class="main-content">
<div id="breadcrumbs" class="breadcrumbs noprint">
<ul class="breadcrumb">
  <li><i class="fa fa-user home-icon active" ></i>  <a href="/account_page.php">dengxiangcheng ( 邓祥成 ) </a>
  <span class="label hidden-xs label-default arrowed">开发人员</span></li>
</ul>
<div class="nav-recent hidden-xs">最近浏览的: <a href="/view.php?id=2491" title="[已发布] 【运营管理-用户统计】今日累计充值用户数量错误" class="resolved">0002491</a>, <a href="/view.php?id=2424" title="[已关闭] 【注册】通过渠道链接注册，未存渠道信息" class="resolved">0002424</a>, <a href="/view.php?id=2486" title="[已解决] 【运营管理-工单消息】鼠标悬停选中样式问题" class="resolved">0002486</a>, <a href="/view.php?id=2283" title="[已关闭] 0002283:+【告警相关】用户云服务器cpu，内存达到告警设置时，没有出现告警" class="resolved">0002283</a>, <a href="/view.php?id=2381" title="[已解决] {需求}【云计算-计算节点】磁盘字段需要删除" class="resolved">0002381</a></div><div id="nav-search" class="nav-search"><form class="form-search" method="post" action="/jump_to_bug.php"><span class="input-icon"><input type="text" name="bug_id" autocomplete="off" class="nav-search-input" placeholder="问题 #"><i class="fa fa-search ace-icon nav-search-icon" ></i></span></form></div>
</div>
  <div class="page-content">
<div class="row">

<div class="col-md-12 col-xs-12">

	<div id="bug-change-status-div" class="form-container">
	<form id="bug-change-status-form" name="bug_change_status_form" method="post" action="bug_update.php">

	<fieldset>

	<input type="hidden" name="bug_update_token" value="20250715WOLsuLb1pnUnlc_TFqERnUJYevqN9p88"/>	<div class="widget-box widget-color-blue2">
	<div class="widget-header widget-header-small">
		<h4 class="widget-title lighter">
			关闭问题		</h4>
	</div>

	<div class="widget-body">
	<div class="widget-main no-padding">

	<div class="table-responsive">
	<table class="table table-bordered table-condensed table-striped">
		<thead>
			<input type="hidden" name="bug_id" value="2491" />
			<input type="hidden" name="status" value="90" />
			<input type="hidden" name="last_updated" value="1752562900" />
					</thead>
		<tbody>
<!-- Assigned To -->
			<tr>
				<th class="category">
					分派给				</th>
				<td>
					<select name="handler_id" class="input-sm">
						<option value="0"></option>
						<option value="1" >administrator</option><option value="19" >liujx</option><option value="4" >Misso</option><option value="2" >余勇德 (yuland)</option><option value="3" >冉伟 (ranwei)</option><option value="20" >刘佳鑫 (liujiaxin)</option><option value="28" >刘思婷 (manman)</option><option value="5" >刘鲜 (liuxian)</option><option value="10" >吴飞鹏 (wufeipeng)</option><option value="7" >周祯人 (zhouzhenren)</option><option value="12" >唐元彬 (tangyuanbin)</option><option value="25" >唐平 (tangping)</option><option value="14" >张桎菱 (zhangzhiling)</option><option value="24" >张茏 (zhanglong)</option><option value="27" >朱兆峰 (zhuzhaofeng)</option><option value="13" >柳文莉 (liuwenli)</option><option value="26" >涂佳杨 (tujiayang)</option><option value="18" >王克 (wangke)</option><option value="16" >王建 (wangjian)</option><option value="11" >田风云 (tianfy)</option><option value="8" >赵义博 (zhaoyibo)</option><option value="9" >邓家祥 (dengjiaxiang)</option><option value="29"  selected="selected">邓祥成 (dengxiangcheng)</option><option value="21" >邱泓瑀 (qiuhongyu@test.com)</option><option value="15" >阮浪 (ruanlang)</option><option value="23" >陈俊 (bigcity)</option><option value="6" >陈全杰 (chenquanjie)</option><option value="17" >雷宇 (laya)</option>					</select>
				</td>
			</tr>


	<!-- Custom Fields -->
			<tr>
				<th class="category">
					查看权限				</th>
				<td>
			<input type="checkbox" id="bugnote_add_view_status" class="ace" name="private"
				 />
			<label class="lbl padding-6" for="bugnote_add_view_status">私有</label>
				</td>
			</tr>
			<!-- Bugnote -->
			<tr id="bug-change-status-note">
				<th class="category">
					添加注释				</th>
				<td>
					<textarea name="bugnote_text" id="bugnote_text" class="form-control" cols="80" rows="7"></textarea>
				</td>
			</tr>

</tbody>
</table>
<input type="hidden" name="action_type" value="close" />

</div>
</div>
<div class="widget-toolbox padding-8 clearfix">
	<span class="required pull-right"> * 必填</span>
	<input type="submit" class="btn btn-primary btn-white btn-round" value="关闭问题" />
</div>
</div>
</div>
</div>
</form>
<div class="space-10"></div>
</div>
<div class="col-md-12 col-xs-12"><div class="widget-box widget-color-blue2"><div class="widget-header widget-header-small"><h4 class="widget-title lighter"><i class="fa fa-bars ace-icon" ></i>查看问题详情</h4></div><div class="widget-body"><div class="widget-toolbox padding-8 clearfix noprint"><div class="btn-group pull-left"><a class="btn btn-primary btn-white btn-round btn-sm" href="#bugnotes">跳转到注释</a><a class="btn btn-primary btn-white btn-round btn-sm" href="#history">跳转到历史</a></div><div class="btn-group pull-right"><a class="btn btn-primary btn-white btn-round btn-sm" href="view.php?id=2424">&gt;&gt;</a></div></div><div class="widget-main no-padding"><div class="table-responsive"><table class="table table-bordered table-condensed"><tbody><tr class="bug-header"><th class="bug-id category width-15">编号</th><th class="bug-project category width-20">项目</th><th class="bug-category category width-15">分类</th><th class="bug-view-status category width-15">查看权限</th><th class="bug-date-submitted category width-15">报告日期</th><th class="bug-last-modified category width-20">最后更新</th></tr><tr class="bug-header-data"><td class="bug-id">0002491</td><td class="bug-project">云计算海外版-管理端</td><td class="bug-category">后端</td><td class="bug-view-status">公开</td><td class="bug-date-submitted">2025-07-15 11:27</td><td class="bug-last-modified">2025-07-15 15:01</td></tr><tr class="spacer"><td colspan="6"></td></tr><tr class="hidden"></tr><tr><th class="bug-reporter category">报告员</th><td class="bug-reporter"><a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>&nbsp;</td><th class="bug-assigned-to category">分派给</th><td class="bug-assigned-to"><a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>&nbsp;</td><td colspan="2">&#160;</td></tr><tr><th class="bug-priority category">优先级</th><td class="bug-priority">中</td><th class="bug-severity category">严重性</th><td class="bug-severity">小错误</td><th class="bug-reproducibility category">出现频率</th><td class="bug-reproducibility">总是</td></tr><tr><th class="bug-status category">状态</th><td class="bug-status"><i class="fa fa-square fa-status-box status-81-fg" ></i> 已发布</td><th class="bug-resolution category">处理状况</th><td class="bug-resolution">已修正</td><td colspan="2">&#160;</td></tr><tr class="spacer"><td colspan="6"></td></tr><tr class="hidden"></tr><tr><th class="bug-summary category">摘要</th><td class="bug-summary" colspan="5">0002491: 【运营管理-用户统计】今日累计充值用户数量错误</td></tr><tr><th class="bug-description category">描述</th><td class="bug-description" colspan="5">【运营管理-用户统计】今日累计充值用户数量错误</td></tr><tr><th class="bug-steps-to-reproduce category">问题重现步骤</th><td class="bug-steps-to-reproduce" colspan="5">1</td></tr><tr><th class="bug-additional-information category">附注</th><td class="bug-additional-information" colspan="5">2</td></tr><tr><th class="bug-tags category">标签</th><td class="bug-tags" colspan="5">没加标签.</td></tr><tr class="noprint"><th class="bug-attach-tags category">附件</th><td class="bug-attach-tags" colspan="5"><div class="well well-xs">
<div id="attachment_preview_2848_open" class=" collapse-open noprint"><a href="file_download.php?file_id=2848&amp;type=bug"><i class="fa fa-file-image-o " title="png文件图标"></i></a>&#32;<a href="file_download.php?file_id=2848&amp;type=bug">image.png</a>&#32;(209,598&#32;字节)&#32;&nbsp;&nbsp;<a id="attachment_preview_2848_open_link" class="collapse-link noprint"><i class="fa fa-chevron-up bigger-120" title="-"></i></a>
<div class="bug-attachment-preview-image"><a href="file_download.php?file_id=2848&amp;type=bug"><img src="file_download.php?file_id=2848&amp;type=bug&amp;show_inline=1&amp;file_show_inline_token=20250715TZqasyvbkncb1b2auVvITjbEr3qVw_rI" alt="" loading="lazy" style="border: 0; max-height:250px;" /></a></div></div>
<div id="attachment_preview_2848_closed" class="collapse-section-closed  collapse-closed"><a href="file_download.php?file_id=2848&amp;type=bug"><i class="fa fa-file-image-o " title="png文件图标"></i></a>&#32;<a href="file_download.php?file_id=2848&amp;type=bug">image.png</a>&#32;(209,598&#32;字节)&#32;&nbsp;&nbsp;<a id="attachment_preview_2848_closed_link" class="collapse-link noprint"><i class="fa fa-chevron-down bigger-120" title="+"></i></a></div></div></td></tr><tr class="spacer"><td colspan="6"></td></tr><tr class="hidden"></tr></tbody></table></div></div></div></div></div>
<div class="col-md-12 col-xs-12">
<a id="attachments"></a>
<a id="bugnotes"></a>
<div class="space-10"></div>

<div id="bugnotes" class="widget-box widget-color-blue2 ">
<div class="widget-header widget-header-small">
	<h4 class="widget-title lighter">
		<i class="fa fa-comments ace-icon" ></i>		活动	</h4>
	<div class="widget-toolbar">
		<a data-action="collapse" href="#">
			<i class="fa fa-chevron-up 1 ace-icon bigger-125" ></i>		</a>
	</div>
	</div>
	<div class="widget-body">
	<div class="widget-main no-padding">
	<div class="table-responsive">
	<table class="table table-bordered table-condensed table-striped">
<tr class="bugnote visible-on-hover-toggle" id="c755">
		<td class="category">
		<div class="pull-left padding-2">		</div>
		<div class="pull-left padding-2">
		<p class="no-margin">
			<i class="fa fa-user grey" ></i> <a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>		</p>
		<p class="no-margin small lighter">
			<i class="fa fa-clock-o grey" ></i>			2025-07-15 14:40					</p>
		<p class="no-margin">
			<span class="label label-sm label-default arrowed-in-right">开发人员</span>			&#160;
						<i class="fa fa-link grey" ></i>			<a rel="bookmark" href="view.php?id=2491#c755" class="lighter" title="注释的链接">
				~0000755			</a>
					</p>
				<div class="clearfix"></div>
		<div class="space-2"></div>
		<div class="btn-group visible-on-hover">
				</div>
		</div>
	</td>
	<td class="bugnote-note bugnote-public">
	3	</td>
</tr>
<tr class="spacer">
	<td colspan="2"></td>
</tr>
</table>
</div>
</div>
</div>
</div>
</div>
	<div class="col-md-12 col-xs-12">
		<a id="history"></a>
		<div class="space-10"></div>
		<div id="history" class="widget-box widget-color-blue2 ">
			<div class="widget-header widget-header-small">
				<h4 class="widget-title lighter">
					<i class="fa fa-history ace-icon" ></i>					问题历史				</h4>
				<div class="widget-toolbar">
					<a data-action="collapse" href="#">
						<i class="fa fa-chevron-up 1 ace-icon bigger-125" ></i>					</a>
				</div>
			</div>

			<div class="widget-body">
				<div class="widget-main no-padding">
					<div class="table-responsive">
	<table class="table table-bordered table-condensed table-hover table-striped">
		<thead>
			<tr>
				<th class="small-caption">
					日期				</th>
				<th class="small-caption">
					用户名				</th>
				<th class="small-caption">
					字段				</th>
				<th class="small-caption">
					更改				</th>
			</tr>
		</thead>

		<tbody>
			<tr>
				<td class="small-caption">
					2025-07-15 11:27				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					新建问题				</td>
				<td class="small-caption">
									</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 11:27				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					状态				</td>
				<td class="small-caption">
					新建 =&gt; 已分配				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 11:27				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					分派给				</td>
				<td class="small-caption">
					 =&gt; 邓祥成				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 11:27				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					添加了以下文件：: image.png				</td>
				<td class="small-caption">
									</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 11:57				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					状态				</td>
				<td class="small-caption">
					已分配 =&gt; 已关闭				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 13:57				</td>
				<td class="small-caption">
					<a title="manman" href="http://bug.test.com/view_user_page.php?id=28">刘思婷</a>				</td>
				<td class="small-caption">
					状态				</td>
				<td class="small-caption">
					已关闭 =&gt; 已分配				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 14:13				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					问题重现步骤已修改				</td>
				<td class="small-caption">
					<a href="bug_revision_view_page.php?rev_id=562#r562">查看修订历史</a>				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 14:19				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					问题重现步骤已修改				</td>
				<td class="small-caption">
					<a href="bug_revision_view_page.php?rev_id=563#r563">查看修订历史</a>				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 14:38				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					问题重现步骤已修改				</td>
				<td class="small-caption">
					<a href="bug_revision_view_page.php?rev_id=564#r564">查看修订历史</a>				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 14:40				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					注释已添加: 0000755				</td>
				<td class="small-caption">
									</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 14:41				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					附注已修改				</td>
				<td class="small-caption">
					<a href="bug_revision_view_page.php?rev_id=566#r566">查看修订历史</a>				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 15:01				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					状态				</td>
				<td class="small-caption">
					已分配 =&gt; 已发布				</td>
			</tr>
			<tr>
				<td class="small-caption">
					2025-07-15 15:01				</td>
				<td class="small-caption">
					<a title="dengxiangcheng" href="http://bug.test.com/view_user_page.php?id=29">邓祥成</a>				</td>
				<td class="small-caption">
					处理状况				</td>
				<td class="small-caption">
					未处理 =&gt; 已修正				</td>
			</tr>
		</tbody>
	</table>
					</div>
				</div>
			</div>
		</div>
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
<i class="fa fa-angle-double-up ace-icon icon-only bigger-110" ></i>
</a>
</div>
	<script type="text/javascript" src="/js/bootstrap-3.4.1.min.js"></script>
	<script type="text/javascript" src="/js/moment-with-locales-2.29.4.min.js"></script>
	<script type="text/javascript" src="/js/bootstrap-datetimepicker-4.17.47.min.js"></script>
	<script type="text/javascript" src="/js/typeahead.jquery-1.3.4.min.js"></script>
	<script type="text/javascript" src="/js/list-2.3.1.min.js"></script>
	<script type="text/javascript" src="/js/ace.min.js"></script>
</body>
</html>
