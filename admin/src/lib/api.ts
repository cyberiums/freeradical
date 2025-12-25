import axios, { AxiosInstance } from 'axios'

const API_BASE_URL = '/api'

// Create axios instance with auth interceptor
const createApiClient = (): AxiosInstance => {
  const instance = axios.create({
    baseURL: API_BASE_URL,
  })

  // Add auth token to all requests
  instance.interceptors.request.use((config) => {
    const token = localStorage.getItem('auth_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  })

  // Handle 401 responses
  instance.interceptors.response.use(
    (response) => response,
    (error) => {
      if (error.response?.status === 401) {
        localStorage.removeItem('auth_token')
        window.location.href = '/login'
      }
      return Promise.reject(error)
    }
  )

  return instance
}

const apiClient = createApiClient()

export const api = {
  // Pages
  pages: {
    list: (params?: any) => apiClient.get('/pages', { params }),
    get: (uuid: string) => apiClient.get(`/pages/${uuid}`),
    create: (data: any) => apiClient.post('/pages', data),
    update: (uuid: string, data: any) => apiClient.put(`/pages/${uuid}`, data),
    delete: (uuid: string) => apiClient.delete(`/pages/${uuid}`),
  },
  
  // Media
  media: {
    list: () => apiClient.get('/media'),
    upload: (formData: FormData) => apiClient.post('/media/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    }),
    delete: (uuid: string) => apiClient.delete(`/media/${uuid}`),
  },
  
  // Modules
  modules: {
    list: () => apiClient.get('/modules'),
    create: (data: any) => apiClient.post('/modules', data),
    update: (id: number, data: any) => apiClient.put(`/modules/${id}`, data),
    delete: (id: number) => apiClient.delete(`/modules/${id}`),
  },
  
  // Categories
  categories: {
    list: () => apiClient.get('/categories'),
    create: (data: any) => apiClient.post('/categories', data),
    update: (id: number, data: any) => apiClient.put(`/categories/${id}`, data),
    delete: (id: number) => apiClient.delete(`/categories/${id}`),
  },
  
  // Auth
  auth: {
    login: (email: string, password: string) => 
      apiClient.post('/login', { email, password }),
    logout: () => apiClient.post('/logout'),
    me: () => apiClient.get('/users/me'),
  },

  // Analytics
  analytics: {
    getSummary: () => apiClient.get('/analytics/summary'),
  }
}

export default api
