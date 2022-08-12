import { useMemo, useState } from 'react'

import { invoke } from '@tauri-apps/api/tauri'

interface User {
  id: number
  username: string
  password: string
  created_at: number
  updated_at: number
}

function App() {
  const [users, setUsers] = useState<User[]>([])

  const onClick = async () => {
    const res = await invoke<string>('get_users')
    const users: User[] = JSON.parse(res)
    setUsers(users)
  }

  const displayUsers = useMemo(() => {
    return (
      <>
        {users.map(user => {
          return (
            <div className='flex space-x-2 justify-center items-center'>
              <p>{user.id}</p>
              <p>{user.username}</p>
              <p>{user.password}</p>
            </div>
          )
        })}
      </>
    )
  }, [users])

  return (
    <div className='w-[100vw] h-[100vh] flex flex-col justify-center items-center bg-slate-300'>
      <p>{displayUsers}</p>
      <button onClick={onClick}>Fetch Word</button>
    </div>
  )
}

export default App
