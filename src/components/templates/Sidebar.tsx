import { memo, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getUserPlaylist } from '../../apis/user';
import { useAuthStore } from '../../store/auth';
import { useUserStore } from '../../store/user';
import LazyImage from '../atoms/Image';

interface SideBarProps {
	className?: string;
	sidebarWidth: number;
	setSidebarWidth: (width: number) => void;
}

const Sidebar: React.FC<SideBarProps> = memo(({ className, sidebarWidth, setSidebarWidth }) => {
	const url = window.location.pathname;
	const navigate = useNavigate();

	const [playlist, setPlaylist] = useState<any[]>([]);
	const [isResizing, setIsResizing] = useState(false);
	const [isVisible, setIsVisible] = useState(false);

	const sidebarRef = useRef<HTMLElement>(null);
	const resizeHandleRef = useRef<HTMLDivElement>(null);
	const startXRef = useRef<number>(0);
	const startWidthRef = useRef<number>(240);

	// 缓存认证状态
	const isLogin = useMemo(() => useAuthStore.getState().isLogin, []);
	const userData = useMemo(() => useAuthStore.getState().userData, []);

	// 最小和最大宽度限制
	const MIN_WIDTH = 180;
	const MAX_WIDTH = 400;

	// 拖拽开始
	const handleMouseDown = useCallback(
		(e: React.MouseEvent) => {
			e.preventDefault();
			setIsResizing(true);
			startXRef.current = e.clientX;
			startWidthRef.current = sidebarWidth;

			document.body.style.cursor = 'col-resize';
			document.body.style.userSelect = 'none';
		},
		[sidebarWidth]
	);

	// 拖拽过程
	const handleMouseMove = useCallback(
		(e: MouseEvent) => {
			if (!isResizing) return;

			const deltaX = e.clientX - startXRef.current;
			const newWidth = Math.min(
				Math.max(startWidthRef.current + deltaX, MIN_WIDTH),
				MAX_WIDTH
			);
			setSidebarWidth(newWidth);
		},
		[isResizing, MIN_WIDTH, MAX_WIDTH, setSidebarWidth]
	);

	// 拖拽结束
	const handleMouseUp = useCallback(() => {
		setIsResizing(false);
		document.body.style.cursor = '';
		document.body.style.userSelect = '';

		// 保存宽度到localStorage
		localStorage.setItem('sidebarWidth', sidebarWidth.toString());
	}, [sidebarWidth]);

	// 导航处理函数
	const handleNavigate = useCallback(
		(path: string) => {
			navigate(path);
		},
		[navigate]
	);

	const handlePlaylistNavigate = useCallback(
		(id: string) => {
			navigate(`/playlist/${id}`);
		},
		[navigate]
	);

	// 获取用户歌单
	const fetchUserPlaylist = useCallback(async () => {
		try {
			const res = await getUserPlaylist();
			setPlaylist(res.playlist || []);
		} catch (error) {
			console.error('获取用户歌单失败:', error);
		}
	}, []);

	// 渲染歌单列表
	const renderedPlaylist = useMemo(() => {
		return playlist
			.map((item: any, index) => {
				// 跳过"我喜欢的音乐"歌单
				if (
					item.name === '我喜欢的音乐' &&
					item.creator.userId === userData?.profile.userId
				) {
					useUserStore.setState({ likePlaylist: item.id });
					return null;
				}

				return (
					<button
						key={item.id || index}
						onClick={() => handlePlaylistNavigate(item.id)}
						className="w-full min-h-12.5 max-h-12.5 flex flex-row items-center justify-start flex-nowrap border-none rounded-[var(--border-radius)] p-2 mx-2 mb-1 bg-transparent cursor-pointer text-1rem text-[var(--text-color)] transition-all duration-200 hover:bg-[var(--button-hover-bg-color)] overflow-hidden text-ellipsis whitespace-nowrap group"
						title={item.name}
					>
						<LazyImage
							src={item.coverImgUrl}
							className="w-8.5 h-8.5 min-w-8.5 mr-2 rounded-8px object-cover transition-transform duration-200 group-hover:scale-105"
							alt={item.name}
						/>
						<span className="flex-1 truncate text-left">{item.name}</span>
					</button>
				);
			})
			.filter(Boolean);
	}, [playlist, userData?.profile.userId, handlePlaylistNavigate]);

	// 初始化
	useEffect(() => {
		setIsVisible(true);

		// 获取用户歌单
		if (isLogin) {
			fetchUserPlaylist();

			// 定时更新歌单
			const timer = setInterval(fetchUserPlaylist, 1000 * 60 * 5);
			return () => clearInterval(timer);
		}
	}, [isLogin, fetchUserPlaylist]);

	// 添加全局鼠标事件监听
	useEffect(() => {
		if (isResizing) {
			document.addEventListener('mousemove', handleMouseMove);
			document.addEventListener('mouseup', handleMouseUp);

			return () => {
				document.removeEventListener('mousemove', handleMouseMove);
				document.removeEventListener('mouseup', handleMouseUp);
			};
		}
	}, [isResizing, handleMouseMove, handleMouseUp]);

	return (
		<>
			<nav
				ref={sidebarRef}
				className={`flex flex-col max-h-[calc(100dvh-75px)] bg-[var(--background-color)] border-r border-[var(--hr-color)]/50 transition-all duration-300 pr-3 ${
					isVisible ? 'translate-x-0 opacity-100' : '-translate-x-full opacity-0'
				} ${className || ''}`}
			>
				{/* Logo区域 */}
				<div
					data-tauri-drag-region
					className="w-full h-10.75 mt-5 flex items-center justify-center mb-4 animate-in slide-in-from-top duration-500"
				>
					<img
						src="/vite.svg"
						alt="Cloubit Logo"
						className="max-w-6.25 max-h-6.25 mr-2"
					/>
					<h1 className="text-lg font-bold text-[var(--text-color)] truncate">Cloubit</h1>
				</div>

				{/* 推荐区域 */}
				<div className="w-full p-2 flex flex-col mb-4 animate-in slide-in-from-left duration-500 delay-100">
					<h1 className="p-0 flex-1 mx-2 text-0.8rem text-[var(--second-text-color)] flex items-center justify-start mb-2">
						推荐
					</h1>
					<button
						className={`w-full h-9.5 flex flex-row items-center justify-start flex-nowrap border-none rounded-[var(--border-radius)] p-2 mx-2 mb-1 cursor-pointer text-1rem transition-all duration-200 ${
							url === '/'
								? 'bg-[var(--button-focus-bg-color)] text-[var(--primary-color)]'
								: 'bg-transparent text-[var(--text-color)] hover:bg-[var(--button-hover-bg-color)]'
						}`}
						onClick={() => handleNavigate('/')}
					>
						<span className="i-solar-home-angle-linear mr-2 text-1.5rem" />
						<span className="truncate">推荐</span>
					</button>
				</div>

				{/* 我的区域 */}
				<div className="w-full p-2 flex flex-col mb-4 animate-in slide-in-from-left duration-500 delay-200">
					<h1 className="p-0 flex-1 mx-2 text-0.8rem text-[var(--second-text-color)] flex items-center justify-start mb-2">
						我的
					</h1>

					{isLogin && (
						<button
							className={`w-full h-9.5 flex flex-row items-center justify-start flex-nowrap border-none rounded-[var(--border-radius)] p-2 mx-2 mb-1 cursor-pointer text-1rem transition-all duration-200 ${
								url === '/playlist/like'
									? 'bg-[var(--button-focus-bg-color)] text-[var(--primary-color)]'
									: 'bg-transparent text-[var(--text-color)] hover:bg-[var(--button-hover-bg-color)]'
							}`}
							onClick={() => handleNavigate('/playlist/like')}
						>
							<span className="i-solar-heart-linear mr-2 text-1.5rem" />
							<span className="truncate">收藏</span>
						</button>
					)}

					<button
						className={`w-full h-9.5 flex flex-row items-center justify-start flex-nowrap border-none rounded-[var(--border-radius)] p-2 mx-2 mb-1 cursor-pointer text-1rem transition-all duration-200 ${
							url === '/history'
								? 'bg-[var(--button-focus-bg-color)] text-[var(--primary-color)]'
								: 'bg-transparent text-[var(--text-color)] hover:bg-[var(--button-hover-bg-color)]'
						}`}
						onClick={() => handleNavigate('/history')}
					>
						<span className="i-solar-history-linear mr-2 text-1.5rem" />
						<span className="truncate">历史</span>
					</button>

					<button
						className={`w-full h-9.5 flex flex-row items-center justify-start flex-nowrap border-none rounded-[var(--border-radius)] p-2 mx-2 mb-1 cursor-pointer text-1rem transition-all duration-200 ${
							url === '/download'
								? 'bg-[var(--button-focus-bg-color)] text-[var(--primary-color)]'
								: 'bg-transparent text-[var(--text-color)] hover:bg-[var(--button-hover-bg-color)]'
						}`}
						onClick={() => handleNavigate('/download')}
					>
						<span className="i-solar-download-minimalistic-linear mr-2 text-1.5rem" />
						<span className="truncate">下载</span>
					</button>
				</div>

				{/* 歌单区域 */}
				{isLogin && (
					<div className="w-full p-2 flex flex-col mb-4 animate-in slide-in-from-left duration-500 delay-200">
						<h1 className="p-0 flex-1 mx-2 text-0.8rem text-[var(--second-text-color)] flex items-center justify-start mb-2">
							歌单
							<span
								className="i-solar-add-circle-linear flex text-1.2rem relative top-0.25 ml-auto transition-colors duration-200 hover:text-[var(--primary-color)] cursor-pointer"
								title="创建歌单"
							/>
						</h1>
						<div className="w-full flex-1 overflow-y-auto [&::-webkit-scrollbar]:w-1 [&::-webkit-scrollbar]:transition-all [&::-webkit-scrollbar]:duration-300 [&::-webkit-scrollbar:hover]:w-1.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-[var(--hr-color)] [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb:hover]:bg-[var(--second-text-color)]">
							{playlist.length === 0 ? (
								<div className="flex flex-col items-center justify-center h-20 text-[var(--second-text-color)] opacity-70">
									<span className="i-solar-music-note-4-line-duotone text-2xl mb-2" />
									<p className="text-sm">暂无歌单</p>
								</div>
							) : (
								<div className="space-y-1">{renderedPlaylist}</div>
							)}
						</div>
					</div>
				)}
			</nav>

			{/* 拖拽调整手柄 - 移动到侧栏外部右侧 */}
			<div
				ref={resizeHandleRef}
				className={`absolute top-0 -right-1 w-2 h-full cursor-col-resize z-10 transition-all duration-200 ${
					isResizing
						? 'bg-[var(--primary-color)]/30'
						: 'bg-transparent hover:bg-[var(--hr-color)]/50'
				}`}
				onMouseDown={handleMouseDown}
				title="拖拽调整侧边栏宽度"
			>
				{/* 可视化拖拽指示器 */}
				<div className="absolute top-1/2 left-1/2 w-1 h-12 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center justify-center space-y-0.5 opacity-0 hover:opacity-100 transition-opacity duration-200">
					<div className="w-0.5 h-1 bg-[var(--second-text-color)] rounded-full" />
					<div className="w-0.5 h-1 bg-[var(--second-text-color)] rounded-full" />
					<div className="w-0.5 h-1 bg-[var(--second-text-color)] rounded-full" />
				</div>

				{/* 拖拽时的视觉反馈线 */}
				{isResizing && (
					<div className="absolute top-0 left-1/2 w-0.5 h-full bg-[var(--primary-color)] -translate-x-1/2 animate-pulse" />
				)}
			</div>
		</>
	);
});

Sidebar.displayName = 'Sidebar';

export default Sidebar;
