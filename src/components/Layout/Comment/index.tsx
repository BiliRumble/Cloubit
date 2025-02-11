import { Comment } from '../../../models/comment';
import Avatar from '../../common/Avatar';
import Time from '../../common/Time';
import styles from './Comment.module.scss';

interface CommentProps {
	comment: Comment;
	onLike?: (commentId: number) => void;
}

const CommentComponent: React.FC<CommentProps> = ({ comment }) => {
	return (
		<div className={styles.comment}>
			<div className={styles.comment__avatar}>
				<Avatar url={comment.user.avatarUrl} name={comment.user.nickname} />
			</div>

			<div className={styles.comment__content}>
				<div className={styles.comment__content__header}>
					<span className={styles.comment__content__header__name}>
						{comment.user.nickname}
					</span>
					<p className={styles.comment__content__header__time}>
						<Time time={comment.time} />
					</p>
				</div>
				<div className={styles.comment__content__body}>{comment.content}</div>
				<div className={styles.footer}></div>
			</div>
		</div>
	);
};

export default CommentComponent;
