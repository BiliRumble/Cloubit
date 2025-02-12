import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { getSongComment } from '../../../apis/comment';
import { getSongDetail } from '../../../apis/song';
import CommentComponent from '../../../components/organisms/Comment';
import MediaHeader from '../../../components/organisms/MediaHeader';
import { Comment } from '../../../models/comment';
import { Song, SongDetail } from '../../../models/song';
import styles from '../Comment.module.scss';

const SongComment = () => {
	const { id } = useParams<{ id: string }>();

	const [comments, setComments] = useState<Comment[]>([]);
	const [musicInfo, setMusicInfo] = useState<Song | null>(null);
	const [loading, setLoading] = useState<boolean>(true);

	useEffect(() => {
		getSongDetail(id as unknown as number).then((res) => {
			if (!res) return;
			setMusicInfo(res[0] || {});
		});
		getSongComment(id as unknown as number).then((res) => {
			setComments(res?.comments || []);
			setLoading(false);
			console.warn(comments);
		});
	}, [id]);

	return (
		<>
			{!loading && (
				<>
					<MediaHeader
						cover={musicInfo?.al.picUrl || ''}
						name={musicInfo?.name || ''}
						enableSearch={false}
						className={styles.comments__header}
					/>
					<div className={styles.comments__body}>
						{comments.map((comment) => (
							<CommentComponent
								key={comment.commentId}
								comment={comment}
								onLike={() => {}}
							/>
						))}
					</div>
				</>
			)}
		</>
	);
};

export default SongComment;
