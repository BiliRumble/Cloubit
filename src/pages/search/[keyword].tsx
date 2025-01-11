import { useNavigate, useParams } from 'react-router-dom';
import styles from './NotFound.module.scss';

const search = () => {
	// react-router-dom v6获取路由参数(/search/:keyword)
	const { keyword } = useParams<{ keyword: string }>();
	const navigate = useNavigate();

	return (
		<div>
			<h1>{keyword}</h1>
		</div>
	);
};

export default search;
