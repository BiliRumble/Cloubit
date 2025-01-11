import styles from './NotFound.module.scss';

const notFound = () => {
	return (
		<div className={styles.notfound}>
			<h1>404</h1>
			<p>Page not found</p>
		</div>
	);
};

export default notFound;
