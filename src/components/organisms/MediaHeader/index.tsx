import { ReactNode } from 'react';
import Chip from '../../atoms/Chip';
import Input from '../../atoms/Input';
import LazyImage from '../../atoms/Image';
import styles from './MediaHeader.module.scss';

interface MediaHeaderProps {
	cover: string;
	name: string;
	description?: string;
	author?: string;
	createTime?: number;
	tags?: string[];
	enableSearch?: boolean;
	footerbar?: ReactNode;
	searchKeyword?: string;
	onSearch?: (value: string) => void;
	className?: string;
	style?: React.CSSProperties;
}

const MediaHeader: React.FC<MediaHeaderProps> = ({
	cover,
	name,
	description,
	author,
	createTime,
	tags,
	enableSearch,
	footerbar,
	searchKeyword,
	onSearch,
	className = '',
	style,
}) => {
	return (
		<div className={styles.mediaHeader + ' ' + className} style={style}>
			<LazyImage src={cover} alt="" className={styles.mediaHeader__cover} />
			<div className={styles.mediaHeader__info}>
				<div className={styles.mediaHeader__info__data}>
					<h1 className={styles.mediaHeader__info__data__title} title={name}>
						{name}
					</h1>
					{description && (
						<h3
							className={styles.mediaHeader__info__data__description}
							title={description}
						>
							{description}
						</h3>
					)}
					<div className={styles.mediaHeader__info__data__details}>
						{author && (
							<span className={styles.mediaHeader__info__data__details__author}>
								<i className="i-solar-user-linear" />
								{author}
							</span>
						)}
						{createTime && (
							<span className={styles.mediaHeader__info__data__details__time}>
								<i className="i-solar-clock-circle-linear" />
								{new Date(createTime).toLocaleDateString()}
							</span>
						)}
					</div>
					{tags && tags.length > 0 && (
						<div className={styles.mediaHeader__info__data__details__tags}>
							{tags?.map((tag: string) => <Chip children={tag} />)}
						</div>
					)}
				</div>
				<div className={styles.mediaHeader__info__operator}>
					{footerbar}
					{enableSearch && onSearch && (
						<div className={styles.mediaHeader__info__operator__search}>
							<Input
								allowType="text"
								placeholder="模糊搜索"
								className={styles.mediaHeader__info__operator__search__input}
								value={searchKeyword || ''}
								onChange={onSearch}
							/>
						</div>
					)}
				</div>
			</div>
		</div>
	);
};

export default MediaHeader;
